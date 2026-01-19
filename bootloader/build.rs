use std::env;
use std::path::PathBuf;

fn main() {
    // Get the path to the kernel binary
    let kernel_path = env::var_os("CARGO_BIN_FILE_UTOPIA_KERNEL_utopia_kernel")
        .map(PathBuf::from)
        .expect("CARGO_BIN_FILE_UTOPIA_KERNEL_utopia_kernel env var not set");

    // Create a BIOS bootable disk image
    let bios_path = out_dir().join("bios.img");
    if let Err(e) = bootloader::BiosBoot::new(&kernel_path)
        .create_disk_image(&bios_path) {
        println!("cargo:warning=Failed to create BIOS image: {}", e);
    } else {
        println!("cargo:rustc-env=BIOS_PATH={}", bios_path.display());
    }

    // Create a UEFI bootable disk image  
    let uefi_path = out_dir().join("uefi.img");
    if let Err(e) = bootloader::UefiBoot::new(&kernel_path)
        .create_disk_image(&uefi_path) {
        println!("cargo:warning=Failed to create UEFI image: {}", e);
    } else {
        println!("cargo:rustc-env=UEFI_PATH={}", uefi_path.display());
    }

    // Rerun if the kernel binary changes
    println!("cargo:rerun-if-changed={}", kernel_path.display());
}

fn out_dir() -> PathBuf {
    PathBuf::from(env::var_os("OUT_DIR").unwrap())
}