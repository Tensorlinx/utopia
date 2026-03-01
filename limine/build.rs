use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

/// WSL 脚本执行超时时间（秒）
const WSL_TIMEOUT_SECS: u64 = 120;
/// Limine 安装超时时间（秒）
const LIMINE_INSTALL_TIMEOUT_SECS: u64 = 30;

fn main() -> anyhow::Result<()> {
    let start_time = Instant::now();
    println!("cargo:warning==== Limine Build Script Starting ===");
    
    let manifest_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    
    println!("cargo:warning=Manifest dir: {}", manifest_dir.display());
    println!("cargo:warning=Output dir: {}", out_dir.display());

    // 获取内核二进制文件路径（从 workspace target 目录）
    let workspace_root = manifest_dir.parent().unwrap();
    let kernel_path = workspace_root
        .join("target")
        .join("x86_64-unknown-none")
        .join("debug")
        .join("utopia_kernel");

    println!("cargo:warning=Looking for kernel at: {}", kernel_path.display());

    if !kernel_path.exists() {
        anyhow::bail!(
            "Kernel binary not found at: {}\n\
             Please build the kernel first with:\n\
             cargo build --target x86_64-unknown-none -p utopia_kernel --no-default-features --features limine",
            kernel_path.display()
        );
    }
    
    println!("cargo:warning=Kernel found: {} bytes", fs::metadata(&kernel_path)?.len());

    // 获取 limine 二进制文件目录
    let limine_binary_dir = find_limine_binary_dir(&manifest_dir)?;
    println!("cargo:warning=Using Limine binaries from: {}", limine_binary_dir.display());

    // 创建磁盘镜像目录结构
    let disk_dir = out_dir.join("disk");
    fs::create_dir_all(&disk_dir)?;

    // 复制内核到磁盘目录
    let kernel_dest = disk_dir.join("boot").join("utopia_kernel");
    fs::create_dir_all(kernel_dest.parent().unwrap())?;
    fs::copy(&kernel_path, &kernel_dest)?;
    println!("cargo:warning=Copied kernel to: {}", kernel_dest.display());

    // 复制 limine 引导文件到磁盘目录
    let limine_boot_dir = disk_dir.join("boot").join("limine");
    fs::create_dir_all(&limine_boot_dir)?;

    // 复制必要的 limine 文件
    let limine_files = [
        "limine-bios.sys",
        "limine.conf",
    ];

    for file in &limine_files {
        let src = limine_binary_dir.join(file);
        let dest = limine_boot_dir.join(file);
        if src.exists() {
            fs::copy(&src, &dest)?;
            println!("cargo:warning=Copied: {}", file);
        } else if file == &"limine.conf" {
            // 从项目目录复制配置文件
            let conf_src = manifest_dir.join("limine.conf");
            fs::copy(&conf_src, &dest)?;
            println!("cargo:warning=Copied limine.conf from project");
        }
    }

    // 创建 EFI 启动目录结构
    let efi_dir = disk_dir.join("EFI").join("BOOT");
    fs::create_dir_all(&efi_dir)?;

    // 复制 UEFI 启动文件
    let uefi_files = [
        "BOOTX64.EFI",
        "BOOTIA32.EFI",
    ];

    for file in &uefi_files {
        let src = limine_binary_dir.join(file);
        let dest = efi_dir.join(file);
        if src.exists() {
            fs::copy(&src, &dest)?;
            println!("cargo:warning=Copied UEFI: {}", file);
        }
    }

    // 创建可启动磁盘镜像（使用 WSL）
    let disk_path = out_dir.join("utopia-limine.img");
    create_disk_image_with_wsl(&disk_path, &disk_dir, &limine_binary_dir)?;

    // 设置环境变量供运行时查询
    println!("cargo:rustc-env=LIMINE_DISK_PATH={}", disk_path.display());
    println!("cargo:rustc-env=LIMINE_KERNEL_PATH={}", kernel_path.display());
    println!("cargo:rerun-if-changed={}", kernel_path.display());
    println!("cargo:rerun-if-changed={}", manifest_dir.join("limine.conf").display());
    
    let elapsed = start_time.elapsed();
    println!("cargo:warning==== Limine Build Script Completed in {:.2}s ===", elapsed.as_secs_f64());

    Ok(())
}

/// 查找 limine 二进制文件目录
fn find_limine_binary_dir(manifest_dir: &PathBuf) -> anyhow::Result<PathBuf> {
    // 首先检查 limine-10.8.2-binary 目录
    let versioned_dir = manifest_dir.join("limine-10.8.2-binary");
    if versioned_dir.exists() && versioned_dir.join("limine-bios.sys").exists() {
        return Ok(versioned_dir);
    }

    // 检查 bin 目录
    let bin_dir = manifest_dir.join("bin");
    if bin_dir.exists() && bin_dir.join("limine-bios.sys").exists() {
        return Ok(bin_dir);
    }

    // 如果没有找到，返回错误
    anyhow::bail!(
        "Limine binaries not found. Please download limine release binaries to limine/limine-10.8.2-binary/ or limine/bin/ directory.\n\
         Download from: https://github.com/limine-bootloader/limine/releases"
    )
}

/// 检查 WSL 是否可用
fn check_wsl_available() -> bool {
    match Command::new("wsl").args(&["echo", "test"]).output() {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}

/// 使用 WSL 创建带 FAT32 文件系统的磁盘镜像
fn create_disk_image_with_wsl(disk_path: &PathBuf, disk_dir: &PathBuf, limine_dir: &PathBuf) -> anyhow::Result<()> {
    println!("cargo:warning=Creating bootable disk image with WSL...");
    
    // 预检查 WSL 是否可用
    if !check_wsl_available() {
        println!("cargo:warning=WSL is not available or not responding");
        println!("cargo:warning=Please ensure WSL is installed and running");
        println!("cargo:warning=Fallback to raw disk image...");
        return create_raw_disk_image(disk_path, disk_dir, limine_dir);
    }
    println!("cargo:warning=WSL is available");

    // 将 Windows 路径转换为 WSL 路径
    let disk_path_wsl = windows_to_wsl_path(disk_path)?;
    let disk_dir_wsl = windows_to_wsl_path(disk_dir)?;
    let _limine_dir_wsl = windows_to_wsl_path(limine_dir)?;

    println!("cargo:warning=WSL disk path: {}", disk_path_wsl);
    println!("cargo:warning=WSL disk dir: {}", disk_dir_wsl);

    // 创建 WSL 脚本 - 只创建文件系统，不安装引导扇区
    let script = format!(r#"
#!/bin/bash
set -e

DISK_IMG="{}"
DISK_DIR="{}"

echo "=== Creating disk image with FAT32 filesystem ==="

# 创建 64MB 磁盘镜像
dd if=/dev/zero of="$DISK_IMG" bs=1M count=64

# 创建分区表和 FAT32 分区
parted -s "$DISK_IMG" mklabel msdos
parted -s "$DISK_IMG" mkpart primary fat32 1MiB 100%
parted -s "$DISK_IMG" set 1 boot on

# 设置循环设备
LOOP_DEV=$(sudo losetup -f --show -P "$DISK_IMG")
echo "Using loop device: $LOOP_DEV"

# 格式化分区为 FAT32
sudo mkfs.fat -F 32 "${{LOOP_DEV}}p1"

# 创建挂载点
MOUNT_POINT=$(mktemp -d)
sudo mount "${{LOOP_DEV}}p1" "$MOUNT_POINT"

# 复制文件到分区
echo "Copying files to disk image..."
sudo cp -r "$DISK_DIR"/* "$MOUNT_POINT/"

# 列出复制的内容
echo "Disk contents:"
ls -la "$MOUNT_POINT/"
ls -la "$MOUNT_POINT/boot/" 2>/dev/null || echo "No boot directory"
ls -la "$MOUNT_POINT/boot/limine/" 2>/dev/null || echo "No limine directory"

# 卸载
sudo umount "$MOUNT_POINT"
rm -rf "$MOUNT_POINT"

# 清理循环设备
sudo losetup -d "$LOOP_DEV"

echo "=== FAT32 filesystem created successfully ==="
echo "Note: Bootloader must be installed separately using Windows limine.exe"
"#, disk_path_wsl, disk_dir_wsl);

    // 将脚本写入临时文件
    let script_path = disk_path.parent().unwrap().join("create_disk.sh");
    fs::write(&script_path, script)?;

    // 运行 WSL 脚本（带超时）
    println!("cargo:warning=Running WSL script (timeout: {}s)...", WSL_TIMEOUT_SECS);
    let wsl_result = run_wsl_script_with_timeout(&script_path, WSL_TIMEOUT_SECS);
    
    // 清理脚本
    let _ = fs::remove_file(&script_path);

    match wsl_result {
        Ok(true) => {
            println!("cargo:warning=FAT32 filesystem created successfully with WSL");
            
            // WSL 脚本成功，现在在 Windows 端安装 Limine 引导扇区
            println!("cargo:warning=Installing Limine bootloader using Windows executable...");
            match install_limine_bootloader(disk_path, limine_dir) {
                Ok(_) => {
                    println!("cargo:warning=Disk image created and bootloader installed successfully!");
                }
                Err(e) => {
                    println!("cargo:warning=Failed to install Limine bootloader: {}", e);
                    println!("cargo:warning=You may need to manually run:");
                    println!("cargo:warning=  limine.exe bios-install {}", disk_path.display());
                }
            }
        }
        Ok(false) => {
            println!("cargo:warning=WSL script timed out after {} seconds", WSL_TIMEOUT_SECS);
            println!("cargo:warning=This may be due to:");
            println!("cargo:warning=  - WSL waiting for sudo password");
            println!("cargo:warning=  - Missing tools (parted, dosfstools) in WSL");
            println!("cargo:warning=  - losetup command hanging");
            println!("cargo:warning=Fallback to raw disk image...");
            create_raw_disk_image(disk_path, disk_dir, limine_dir)?;
        }
        Err(e) => {
            println!("cargo:warning=Failed to run WSL: {}", e);
            println!("cargo:warning=Error details: {:?}", e);
            println!("cargo:warning=Fallback to raw disk image...");
            create_raw_disk_image(disk_path, disk_dir, limine_dir)?;
        }
    }

    Ok(())
}

/// 运行 WSL 脚本并设置超时
fn run_wsl_script_with_timeout(script_path: &PathBuf, timeout_secs: u64) -> anyhow::Result<bool> {
    let script_path_wsl = windows_to_wsl_path(script_path)?;
    
    println!("cargo:warning=Starting WSL process...");
    println!("cargo:warning=Script path (WSL): {}", script_path_wsl);
    
    let mut child = Command::new("wsl")
        .args(&["bash", &script_path_wsl])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| anyhow::anyhow!("Failed to spawn WSL process: {}", e))?;
    
    let start = Instant::now();
    let timeout = Duration::from_secs(timeout_secs);
    
    println!("cargo:warning=Waiting for WSL script to complete...");
    
    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                println!("cargo:warning=WSL process exited with status: {:?}", status);
                if status.success() {
                    return Ok(true);
                } else {
                    return Err(anyhow::anyhow!("WSL script failed with status: {:?}", status));
                }
            }
            Ok(None) => {
                // 进程仍在运行
                if start.elapsed() > timeout {
                    println!("cargo:warning=Timeout reached! Killing WSL process...");
                    let _ = child.kill();
                    return Ok(false);
                }
                std::thread::sleep(Duration::from_millis(100));
            }
            Err(e) => {
                return Err(anyhow::anyhow!("Error waiting for WSL process: {}", e));
            }
        }
    }
}

/// 使用 Windows 的 limine.exe 安装引导扇区（带超时）
fn install_limine_bootloader(disk_path: &PathBuf, limine_dir: &PathBuf) -> anyhow::Result<()> {
    let limine_exe = limine_dir.join("limine.exe");
    
    if !limine_exe.exists() {
        anyhow::bail!("limine.exe not found at: {}", limine_exe.display());
    }
    
    println!("cargo:warning=Using limine.exe: {}", limine_exe.display());
    println!("cargo:warning=Installing to disk: {}", disk_path.display());
    println!("cargo:warning=Timeout: {}s", LIMINE_INSTALL_TIMEOUT_SECS);
    
    let mut child = Command::new(&limine_exe)
        .args(&["bios-install", disk_path.to_str().unwrap()])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| anyhow::anyhow!("Failed to spawn limine.exe: {}", e))?;
    
    let start = Instant::now();
    let timeout = Duration::from_secs(LIMINE_INSTALL_TIMEOUT_SECS);
    
    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                if status.success() {
                    println!("cargo:warning=Limine bootloader installed successfully!");
                    return Ok(());
                } else {
                    anyhow::bail!("limine bios-install failed with status: {:?}", status);
                }
            }
            Ok(None) => {
                if start.elapsed() > timeout {
                    println!("cargo:warning=Limine install timeout! Killing process...");
                    let _ = child.kill();
                    anyhow::bail!("Limine installation timed out after {} seconds", LIMINE_INSTALL_TIMEOUT_SECS);
                }
                std::thread::sleep(Duration::from_millis(50));
            }
            Err(e) => {
                anyhow::bail!("Error waiting for limine.exe: {}", e);
            }
        }
    }
}

/// 将 Windows 路径转换为 WSL 路径
fn windows_to_wsl_path(path: &PathBuf) -> anyhow::Result<String> {
    let path_str = path.to_str().ok_or_else(|| anyhow::anyhow!("Invalid path"))?;

    // 转换 C:\Users\... 为 /mnt/c/Users/...
    if path_str.len() >= 2 && path_str.chars().nth(1) == Some(':') {
        let drive = path_str.chars().next().unwrap().to_lowercase().to_string();
        let rest = &path_str[2..].replace('\\', "/");
        Ok(format!("/mnt/{}{}", drive, rest))
    } else {
        Ok(path_str.replace('\\', "/"))
    }
}

/// 创建原始磁盘镜像（回退方案）
fn create_raw_disk_image(disk_path: &PathBuf, _disk_dir: &PathBuf, limine_dir: &PathBuf) -> anyhow::Result<()> {
    // 创建空的磁盘镜像文件 (64MB)
    let disk_size = 64 * 1024 * 1024; // 64MB
    let disk_file = fs::File::create(disk_path)?;

    // 分配空间（填充零）
    disk_file.set_len(disk_size as u64)?;
    drop(disk_file);

    println!("cargo:warning=Created raw disk image: {} ({} MB)", disk_path.display(), disk_size / 1024 / 1024);

    // 尝试使用 limine 工具安装引导扇区
    let limine_exe = limine_dir.join("limine.exe");
    if limine_exe.exists() {
        println!("cargo:warning=Installing Limine bootloader to disk...");

        let result = Command::new(&limine_exe)
            .args(&["bios-install", disk_path.to_str().unwrap()])
            .status();

        match result {
            Ok(status) if status.success() => {
                println!("cargo:warning=Limine bootloader installed successfully");
            }
            Ok(status) => {
                println!("cargo:warning=limine bios-install exited with status: {:?}", status);
            }
            Err(e) => {
                println!("cargo:warning=Failed to run limine.exe: {}", e);
            }
        }
    }

    Ok(())
}
