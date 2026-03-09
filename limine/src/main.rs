use std::process::Command;

fn main() {
    // Get the kernel path from the build script
    let kernel_path = env!("LIMINE_KERNEL_PATH");
    let disk_path = env!("LIMINE_DISK_PATH");

    println!("Starting Utopia OS with Limine bootloader...");
    println!("Kernel: {}", kernel_path);
    println!("Disk image: {}", disk_path);

    // Boot from disk image with Limine
    run_with_disk_image(disk_path);
}

fn run_with_disk_image(disk_path: &str) {
    println!("Running with Limine disk image...");

    // Run QEMU with the disk image
    let mut cmd = Command::new("C:\\Program Files\\qemu\\qemu-system-x86_64.exe");

    // Memory
    cmd.arg("-m")
       .arg("512M");

    // Disk image
    cmd.arg("-drive")
       .arg(format!("format=raw,file={}", disk_path));

    // Boot from disk
    cmd.arg("-boot")
       .arg("order=c");

    // Serial output
    cmd.arg("-serial")
       .arg("stdio");

    // VGA
    cmd.arg("-vga")
       .arg("std");

    println!("QEMU command: {:?}", cmd);

    let status = cmd.status().expect("Failed to run QEMU");

    if !status.success() {
        eprintln!("QEMU exited with error code: {:?}", status.code());
        std::process::exit(1);
    }
}
