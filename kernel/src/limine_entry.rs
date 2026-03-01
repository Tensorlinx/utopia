//! Limine 引导加载程序入口点
//! 当使用 Limine 作为引导加载程序时使用此入口

#![cfg(feature = "limine")]

use core::arch::asm;
use core::panic::PanicInfo;
use crate::boot_info::{BootInfo, FrameBufferInfo, PixelFormat, MemoryRegion};

// Limine 协议魔数和版本
const LIMINE_COMMON_MAGIC: [u64; 2] = [0xc7b1dd30df4c8b88, 0x0a82e883a194f07b];
const LIMINE_FRAMEBUFFER_REQUEST: [u64; 4] = [
    LIMINE_COMMON_MAGIC[0], LIMINE_COMMON_MAGIC[1],
    0x9d5827dcd881dd75, 0xa3148604f6fab11b
];
const LIMINE_MEMMAP_REQUEST: [u64; 4] = [
    LIMINE_COMMON_MAGIC[0], LIMINE_COMMON_MAGIC[1],
    0x67cf3d9d378a806f, 0xe304acdfc50c3c62
];
const LIMINE_RSDP_REQUEST: [u64; 4] = [
    LIMINE_COMMON_MAGIC[0], LIMINE_COMMON_MAGIC[1],
    0xc5e77b6b397e7b21, 0x9e421c1053fdd180
];

// Limine 请求结构 - 使用 UnsafeCell 来允许可变静态
#[repr(C)]
pub struct LimineRequest {
    id: [u64; 4],
    revision: u64,
    response: *mut (),
}

// 实现 Sync，因为 Limine 协议要求这些静态变量
unsafe impl Sync for LimineRequest {}

impl LimineRequest {
    pub const fn new(id: [u64; 4]) -> Self {
        Self {
            id,
            revision: 0,
            response: core::ptr::null_mut(),
        }
    }
}

// 静态请求实例
#[used]
#[link_section = ".requests"]
static FRAMEBUFFER_REQUEST: LimineRequest = LimineRequest::new(LIMINE_FRAMEBUFFER_REQUEST);

#[used]
#[link_section = ".requests"]
static MEMMAP_REQUEST: LimineRequest = LimineRequest::new(LIMINE_MEMMAP_REQUEST);

#[used]
#[link_section = ".requests"]
static RSDP_REQUEST: LimineRequest = LimineRequest::new(LIMINE_RSDP_REQUEST);

// Limine 帧缓冲区响应
#[repr(C)]
pub struct LimineFramebufferResponse {
    revision: u64,
    framebuffer_count: u64,
    framebuffers: *mut *mut LimineFramebuffer,
}

#[repr(C)]
pub struct LimineFramebuffer {
    pub address: *mut u8,
    pub width: u64,
    pub height: u64,
    pub pitch: u64,
    pub bpp: u16,
    pub memory_model: u8,
    pub red_mask_size: u8,
    pub red_mask_shift: u8,
    pub green_mask_size: u8,
    pub green_mask_shift: u8,
    pub blue_mask_size: u8,
    pub blue_mask_shift: u8,
    pub unused: [u8; 7],
    pub edid_size: u64,
    pub edid: *mut (),
}

// Limine 内存映射响应
#[repr(C)]
pub struct LimineMemmapResponse {
    revision: u64,
    entry_count: u64,
    entries: *mut *mut LimineMemmapEntry,
}

#[repr(C)]
pub struct LimineMemmapEntry {
    pub base: u64,
    pub length: u64,
    pub entry_type: u64,
}

// Limine RSDP 响应
#[repr(C)]
pub struct LimineRsdpResponse {
    revision: u64,
    address: *mut (),
}

/// Limine 启动信息结构
pub struct LimineBootInfo {
    pub framebuffer: Option<&'static LimineFramebuffer>,
    pub rsdp: Option<u64>,
}

impl BootInfo for LimineBootInfo {
    fn framebuffer_info(&self) -> Option<FrameBufferInfo> {
        self.framebuffer.map(|fb| {
            let pixel_format = if fb.memory_model == 1 {
                if fb.red_mask_shift == 16 {
                    PixelFormat::Rgb
                } else {
                    PixelFormat::Bgr
                }
            } else {
                PixelFormat::Unknown
            };

            FrameBufferInfo {
                width: fb.width as usize,
                height: fb.height as usize,
                stride: fb.pitch as usize / ((fb.bpp as usize + 7) / 8),
                pixel_format,
                bytes_per_pixel: (fb.bpp as usize + 7) / 8,
                physical_address: fb.address as usize,
            }
        })
    }

    fn framebuffer_address(&self) -> Option<u64> {
        self.framebuffer.map(|fb| fb.address as u64)
    }

    fn memory_regions(&self) -> &[MemoryRegion] {
        &[]
    }

    fn rsdp_address(&self) -> Option<u64> {
        self.rsdp
    }

    fn command_line(&self) -> Option<&str> {
        None
    }
}

/// 内核入口点
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // 初始化串口（用于早期调试）
    unsafe {
        crate::serial::init_serial_early();
    }

    // 解析 Limine 提供的信息
    let boot_info = parse_limine_info();

    // 跳转到内核主函数
    crate::kernel_main_limine(&boot_info);
}

/// 解析 Limine 引导信息
fn parse_limine_info() -> LimineBootInfo {
    let framebuffer = unsafe {
        if !FRAMEBUFFER_REQUEST.response.is_null() {
            let response = &*(FRAMEBUFFER_REQUEST.response as *mut LimineFramebufferResponse);
            if response.framebuffer_count > 0 && !response.framebuffers.is_null() {
                let fb_ptr = *response.framebuffers;
                if !fb_ptr.is_null() {
                    Some(&*fb_ptr)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    };

    let rsdp = unsafe {
        if !RSDP_REQUEST.response.is_null() {
            let response = &*(RSDP_REQUEST.response as *mut LimineRsdpResponse);
            if !response.address.is_null() {
                Some(response.address as u64)
            } else {
                None
            }
        } else {
            None
        }
    };

    LimineBootInfo {
        framebuffer,
        rsdp,
    }
}

/// 获取帧缓冲区可变引用
pub fn get_framebuffer_mut() -> Option<&'static mut LimineFramebuffer> {
    unsafe {
        if !FRAMEBUFFER_REQUEST.response.is_null() {
            let response = &*(FRAMEBUFFER_REQUEST.response as *mut LimineFramebufferResponse);
            if response.framebuffer_count > 0 && !response.framebuffers.is_null() {
                let fb_ptr = *response.framebuffers;
                if !fb_ptr.is_null() {
                    return Some(&mut *fb_ptr);
                }
            }
        }
        None
    }
}

/// Limine panic 处理程序
#[cfg(feature = "limine")]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // 尝试使用串口输出 panic 信息
    let _ = crate::serial::_print(format_args!("KERNEL PANIC: {}\n", info));

    // 无限循环
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}
