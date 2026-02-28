use std::process::Command;
use std::env;
use std::path::PathBuf;

fn main() {
    // Determine boot mode based on compile-time feature
    let use_uefi = cfg!(feature = "uefi_mode");
    
    // Get the appropriate disk image path from the build script
    let disk_path = if use_uefi {
        env!("UEFI_PATH")
    } else {
        env!("BIOS_PATH")
    };
    
    println!("Starting Utopia OS in {} mode...", if use_uefi { "UEFI" } else { "BIOS" });
    println!("Using disk image: {}", disk_path);
    
    // Run QEMU with the disk image and serial output
    let mut cmd = Command::new("C:\\Program Files\\qemu\\qemu-system-x86_64.exe");
    
    // Add UEFI firmware if in UEFI mode
    if use_uefi {
        // Try to find OVMF firmware in common locations
        let ovmf_code_paths = [
            "C:\\Program Files\\qemu\\share\\edk2-x86_64-code.fd",
            "C:\\Program Files\\qemu\\share\\OVMF_CODE.fd",
            "C:\\Program Files\\qemu\\share\\ovmf-x64\\OVMF_CODE.fd",
        ];
        
        let ovmf_vars_paths = [
            "C:\\Program Files\\qemu\\share\\edk2-x86_64-vars.fd",
            "C:\\Program Files\\qemu\\share\\OVMF_VARS.fd",
            "C:\\Program Files\\qemu\\share\\ovmf-x64\\OVMF_VARS.fd",
        ];
        
        let mut ovmf_code_found = false;
        let mut ovmf_code_path = "";
        for path in &ovmf_code_paths {
            if std::path::Path::new(path).exists() {
                ovmf_code_path = path;
                ovmf_code_found = true;
                println!("Using UEFI CODE firmware: {}", path);
                break;
            }
        }
        
        let mut ovmf_vars_found = false;
        let mut ovmf_vars_path = "";
        for path in &ovmf_vars_paths {
            if std::path::Path::new(path).exists() {
                ovmf_vars_path = path;
                ovmf_vars_found = true;
                println!("Using UEFI VARS firmware: {}", path);
                break;
            }
        }
        
        if ovmf_code_found {
            // Use pflash for UEFI firmware (required for modern QEMU)
            cmd.arg("-drive")
               .arg(format!("if=pflash,format=raw,unit=0,readonly=on,file={}", ovmf_code_path));
            
            if ovmf_vars_found {
                cmd.arg("-drive")
                   .arg(format!("if=pflash,format=raw,unit=1,file={}", ovmf_vars_path));
            } else {
                // Create a temporary vars file if not found
                let temp_vars = create_temp_vars_file();
                cmd.arg("-drive")
                   .arg(format!("if=pflash,format=raw,unit=1,file={}", temp_vars.display()));
            }
        } else {
            eprintln!("Warning: UEFI firmware (OVMF) not found. UEFI boot may not work.");
            eprintln!("Please install OVMF firmware for your platform.");
            eprintln!("Searched paths:");
            for path in &ovmf_code_paths {
                eprintln!("  - {}", path);
            }
        }
    }
    
    // Add the OS disk image
    cmd.arg("-drive")
       .arg(format!("format=raw,file={}", disk_path));
    
    // Serial output
    cmd.arg("-serial")
       .arg("stdio");
    
    let status = cmd.status().expect("Failed to run QEMU");
    
    if !status.success() {
        eprintln!("QEMU exited with error code: {:?}", status.code());
        std::process::exit(1);
    }
}

fn create_temp_vars_file() -> PathBuf {
    let temp_dir = env::temp_dir();
    let vars_path = temp_dir.join("ovmf_vars_tmp.fd");
    
    // Create a 64KB empty file for UEFI variables
    if !vars_path.exists() {
        std::fs::write(&vars_path, vec![0u8; 65536]).expect("Failed to create temp vars file");
    }
    
    vars_path
}
