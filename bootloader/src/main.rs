use std::process::Command;
use std::env;

fn main() {
    // Get the path to the BIOS disk image from the build script
    let bios_path = env!("BIOS_PATH");
    
    // Run QEMU with the BIOS disk image and serial output
    let mut cmd = Command::new("C:\\Program Files\\qemu\\qemu-system-x86_64.exe");
    cmd.arg("-drive")
       .arg(format!("format=raw,file={}", bios_path))
       .arg("-serial")
       .arg("stdio");
    
    let status = cmd.status().expect("Failed to run QEMU");
    
    if !status.success() {
        eprintln!("QEMU exited with error code: {:?}", status.code());
        std::process::exit(1);
    }
}