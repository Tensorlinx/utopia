#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use crate::constants::qemu::*;

mod serial;
mod logging;
mod constants;
mod error;

// 引导信息抽象层
pub mod boot_info;

// 根据特性标志选择引导方式
#[cfg(feature = "limine")]
pub mod limine_entry;

// Multiboot 2 支持
#[cfg(feature = "multiboot2")]
pub mod multiboot2;

// 默认使用 bootloader_api（向后兼容）
#[cfg(not(any(feature = "limine", feature = "multiboot2")))]
use bootloader_api::{BootInfo, entry_point};

/// 帧缓冲区包装类型
pub struct FrameBufferWrapper {
    pub buffer: &'static mut [u8],
    pub info: FrameBufferInfo,
}

/// 帧缓冲区信息
pub struct FrameBufferInfo {
    pub width: usize,
    pub height: usize,
    pub stride: usize,
    pub pixel_format: boot_info::PixelFormat,
    pub bytes_per_pixel: usize,
}

/// 内核主函数 - 被引导加载程序调用 (bootloader_api)
#[cfg(not(any(feature = "limine", feature = "multiboot2")))]
entry_point!(kernel_main);

#[cfg(not(any(feature = "limine", feature = "multiboot2")))]
fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    // 从 bootloader_api 的 BootInfo 中提取帧缓冲区
    let _framebuffer = boot_info.framebuffer.as_mut().expect("No framebuffer provided");
    
    // 调用通用的内核初始化
    kernel_init_common();
}

/// Limine 引导入口点
#[cfg(feature = "limine")]
pub fn kernel_main_limine(_boot_info: &limine_entry::LimineBootInfo) -> ! {
    // 调用通用的内核初始化
    kernel_init_common();
}

/// Multiboot 2 引导入口点
#[cfg(feature = "multiboot2")]
pub fn kernel_main_multiboot2(_boot_info: &multiboot2::Multiboot2BootInfo) -> ! {
    // 调用通用的内核初始化
    kernel_init_common();
}

/// 通用的内核初始化函数
fn kernel_init_common() -> ! {
    // 初始化日志记录器
    if let Err(e) = logging::init() {
        panic!("Failed to initialize logger: {}", e);
    }

    #[cfg(feature = "limine")]
    log::info!("Kernel initialized with Limine bootloader!");
    
    #[cfg(feature = "multiboot2")]
    log::info!("Kernel initialized with Multiboot 2!");
    
    #[cfg(not(any(feature = "limine", feature = "multiboot2")))]
    log::info!("Kernel initialized with bootloader_api!");

    // 内核启动完成提示
    let startup_messages = [
        "=== UTOPIA KERNEL STARTED ===",
        "STEP 1: LOGGING INIT OK",
        "STEP 2: KERNEL RUNNING",
        #[cfg(feature = "limine")]
        "STEP 3: LIMINE BOOT SUCCESS",
        #[cfg(feature = "multiboot2")]
        "STEP 3: MULTIBOOT 2 BOOT SUCCESS",
        #[cfg(not(any(feature = "limine", feature = "multiboot2")))]
        "STEP 3: BOOTLOADER_API BOOT SUCCESS",
        "STEP 4: ALL SYSTEMS OK!",
        "=========================",
        "STEP 5: ENTERING MAIN LOOP..."
    ];
    
    for message in &startup_messages {
        log::info!("{}", message);
    }
    
    log::info!("All startup messages completed");
    
    // 进入主循环（不会返回）
    kernel_main_loop();
}

/// 内核主循环
fn kernel_main_loop() -> ! {
    // 使用hlt指令的无限循环，避免CPU占用过高
    loop {
        x86_64::instructions::hlt();
    }
}

/// This function is called on panic in non-test mode.
#[cfg(not(test))]
#[cfg(not(any(feature = "limine", feature = "multiboot2")))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
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
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("[failed]\n");
    println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}
