#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use bootloader_api::{BootInfo, entry_point};
use crate::constants::qemu::*;

mod vga_buffer;
mod serial;
mod logging;
mod logo;
mod constants;
mod font;
mod error;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    // 初始化日志记录器
    if let Err(e) = logging::init() {
        // 如果日志初始化失败，使用panic处理
        panic!("Failed to initialize logger: {}", e);
    }

    // 获取framebuffer信息
    let framebuffer = boot_info.framebuffer.as_mut().expect("No framebuffer provided");
    log::info!("Framebuffer: {}x{}, format: {:?}", 
        framebuffer.info().width, framebuffer.info().height, framebuffer.info().pixel_format);

    // 初始化帧缓冲区文本输出
    if let Err(e) = vga_buffer::init_vga(framebuffer) {
        panic!("Failed to initialize VGA: {}", e);
    }
    log::info!("VGA initialized");

    // 打印LOGO
    logo::print_logo();
    log::info!("Kernel started!");

    // 内核启动完成提示
    let startup_messages = [
        "=== UTOPIA KERNEL STARTED ===",
        "STEP 1: VGA INIT OK",
        "STEP 2: KERNEL RUNNING", 
        "STEP 3: VGA OUTPUT TEST",
        "STEP 4: ALL SYSTEMS OK!",
        "=========================",
        "STEP 5: ENTERING MAIN LOOP..."
    ];
    
    for message in &startup_messages {
        println!("{}", message);
        log::info!("{}", message);
    }
    
    log::info!("All startup messages completed");
    // 使用hlt指令的无限循环，避免CPU占用过高
    loop {
        x86_64::instructions::hlt();
    }
}

/// This function is called on panic in non-test mode.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    log::error!("[PANIC] {}", info);
    // 禁用中断并halt
    x86_64::instructions::interrupts::disable();
    loop {
        x86_64::instructions::hlt();
    }
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = EXIT_SUCCESS,
    Failed = EXIT_FAILED,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(EXIT_PORT);
        port.write(exit_code as u32);
    }
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("[failed]\n");
    println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}