//! Limine 引导协议支持 (Stivale2)
//! 用于兼容 Limine 引导加载器的原生 64 位协议

#![cfg(feature = "limine")]

use core::arch::asm;
use core::panic::PanicInfo;
use crate::boot_info::{BootInfo, FrameBufferInfo, MemoryRegion};

/// Stivale2 头魔数
const STIVALE2_HEADER_MAGIC: u64 = 0x73746976616c6532; // "stivale2"

/// Stivale2 引导信息魔数
const STIVALE2_BOOTLOADER_MAGIC: u64 = 0xc7b1dd30df4c8b88;

/// Stivale2 头结构
/// 必须在 ELF 文件的前 64KB 内，并且 16 字节对齐
#[repr(C, align(16))]
pub struct Stivale2Header {
    /// 必须是 STIVALE2_HEADER_MAGIC
    pub magic: u64,
    /// 入口点地址（0 表示使用 ELF 入口点）
    pub entry: u64,
    /// 栈指针（必须提供有效的栈）
    pub stack: u64,
    /// 标志位
    pub flags: u64,
    /// 标签（指向一个链表，用于请求特定功能）
    pub tags: u64,
}

/// 静态栈（用于 Limine 引导）
/// 放在 .bss 段中，在链接时分配
#[used]
#[link_section = ".bss"]
static mut LIMINE_STACK: [u8; 65536] = [0; 65536];

/// Stivale2 头 - 放在 .stivale2hdr 段中
/// 栈地址会在启动时由汇编代码设置
#[used]
#[link_section = ".stivale2hdr"]
pub static STIVALE2_HEADER: Stivale2Header = Stivale2Header {
    magic: STIVALE2_HEADER_MAGIC,
    entry: 0, // 使用 ELF 入口点
    stack: 0, // 将在运行时设置，或者使用 Limine 提供的栈
    flags: 0,
    tags: 0, // 不请求特定功能
};

/// 直接写入串口端口（不依赖任何初始化）
pub unsafe fn write_serial_direct(c: u8) {
    // 等待串口就绪 (COM1)
    while (core::ptr::read_volatile(0x3FD as *const u8) & 0x20) == 0 {}
    // 写入字符
    core::ptr::write_volatile(0x3F8 as *mut u8, c);
}

/// 直接打印字符串（用于早期调试）
pub unsafe fn print_early(s: &str) {
    for c in s.bytes() {
        if c == b'\n' {
            write_serial_direct(b'\r');
        }
        write_serial_direct(c);
    }
}

/// 将数字转换为十六进制字符串并输出
pub unsafe fn print_hex(val: u64) {
    const HEX_CHARS: &[u8] = b"0123456789abcdef";
    write_serial_direct(b'0');
    write_serial_direct(b'x');
    for i in (0..64).step_by(4).rev() {
        let nibble = ((val >> i) & 0xF) as usize;
        write_serial_direct(HEX_CHARS[nibble]);
    }
}

/// Limine 入口点
/// 
/// 根据 Stivale2 协议，引导加载程序会在 64 位长模式下调用内核入口点。
/// 参数通过寄存器传递（具体取决于引导加载程序的实现）。
/// 
/// Limine 通常会在 RAX 中传递引导信息结构的指针。
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // 获取引导信息指针（Limine 通常通过 RAX 传递）
    let boot_info_ptr: u64;
    unsafe {
        asm!(
            "mov {}, rax",
            out(reg) boot_info_ptr,
            options(nomem, nostack)
        );
    }
    
    // 直接输出调试信息（不依赖任何初始化）
    unsafe {
        print_early("\n=== Limine Entry ===\n");
        print_early("Boot info ptr: ");
        print_hex(boot_info_ptr);
        print_early("\n");
    }

    // 验证魔数（如果引导信息结构有效）
    if boot_info_ptr != 0 {
        let magic = unsafe { core::ptr::read_volatile(boot_info_ptr as *const u64) };
        unsafe {
            print_early("Magic: ");
            print_hex(magic);
            print_early("\n");
        }
        
        if magic == STIVALE2_BOOTLOADER_MAGIC {
            unsafe {
                print_early("Magic OK!\n");
            }
        } else {
            unsafe {
                print_early("WARNING: Unexpected magic, but continuing...\n");
            }
        }
    }

    unsafe {
        print_early("Jumping to kernel main...\n");
    }

    // 创建简单的引导信息
    let boot_info = LimineBootInfo::new(boot_info_ptr);

    // 跳转到内核主函数
    crate::kernel_main_limine(&boot_info);
}

/// Limine 引导信息结构
pub struct LimineBootInfo {
    raw_ptr: u64,
}

impl LimineBootInfo {
    pub fn new(ptr: u64) -> Self {
        Self { raw_ptr: ptr }
    }
}

impl BootInfo for LimineBootInfo {
    fn framebuffer_info(&self) -> Option<FrameBufferInfo> {
        // TODO: 从 Limine 引导信息中解析帧缓冲信息
        None
    }

    fn framebuffer_address(&self) -> Option<u64> {
        // TODO: 从 Limine 引导信息中解析帧缓冲地址
        None
    }

    fn memory_regions(&self) -> &[MemoryRegion] {
        // TODO: 从 Limine 引导信息中解析内存映射
        &[]
    }

    fn rsdp_address(&self) -> Option<u64> {
        // TODO: 从 Limine 引导信息中解析 RSDP 地址
        None
    }

    fn command_line(&self) -> Option<&str> {
        // TODO: 从 Limine 引导信息中解析命令行参数
        None
    }
}

/// Panic 处理程序
#[cfg(feature = "limine")]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        print_early("\n=== KERNEL PANIC ===\n");
        // 尝试输出 panic 信息（简化版）
        print_early("Panic occurred!\n");
    }

    // 无限循环
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}
