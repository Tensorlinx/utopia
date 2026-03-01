//! Multiboot 2 协议支持
//! 用于兼容 Limine 等支持 Multiboot 2 的引导加载器

#![cfg(feature = "multiboot2")]

use core::arch::asm;
use core::panic::PanicInfo;
use crate::boot_info::{BootInfo, FrameBufferInfo, PixelFormat, MemoryRegion};

/// Multiboot 2 魔数
const MULTIBOOT2_MAGIC: u32 = 0xe85250d6;
/// Multiboot 2 架构 - i386
const MULTIBOOT2_ARCHITECTURE_I386: u32 = 0;
/// Multiboot 2 头长度
const MULTIBOOT2_HEADER_LENGTH: u32 = 24;

/// Multiboot 2 信息标签类型
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum TagType {
    End = 0,
    CommandLine = 1,
    BootLoaderName = 2,
    Module = 3,
    BasicMemInfo = 4,
    BiosBootDevice = 5,
    MemoryMap = 6,
    VbeInfo = 7,
    FramebufferInfo = 8,
    ElfSections = 9,
    ApmTable = 10,
    Efi32BitSystemTablePtr = 11,
    Efi64BitSystemTablePtr = 12,
    SmbiosTables = 13,
    AcpiOldRsdp = 14,
    AcpiNewRsdp = 15,
    NetworkingInfo = 16,
    EfiMemoryMap = 17,
    EfiBootServicesNotExited = 18,
    Efi32BitImageHandlePtr = 19,
    Efi64BitImageHandlePtr = 20,
    LoadBaseAddr = 21,
}

/// Multiboot 2 头
#[repr(C, align(8))]
pub struct Multiboot2Header {
    magic: u32,
    architecture: u32,
    header_length: u32,
    checksum: u32,
    end_tag: Multiboot2HeaderTag,
}

/// Multiboot 2 头标签
#[repr(C, align(8))]
struct Multiboot2HeaderTag {
    tag_type: u16,
    flags: u16,
    size: u32,
}

/// Multiboot 2 信息标签基础结构
#[repr(C)]
pub struct Tag {
    pub tag_type: u32,
    pub size: u32,
}

/// Multiboot 2 信息结构
#[repr(C)]
pub struct Multiboot2Info {
    pub total_size: u32,
    _reserved: u32,
    // 标签跟随在这里
}

/// 帧缓冲区信息标签
#[repr(C)]
pub struct FramebufferTag {
    pub tag_type: u32,
    pub size: u32,
    pub addr: u64,
    pub pitch: u32,
    pub width: u32,
    pub height: u32,
    pub bpp: u8,
    pub framebuffer_type: u8,
    pub reserved: u8,
    // 颜色信息跟随在这里
}

/// 内存映射条目
#[repr(C)]
pub struct MemoryMapEntry {
    pub base_addr: u64,
    pub length: u64,
    pub entry_type: u32,
    pub reserved: u32,
}

/// 内存映射标签
#[repr(C)]
pub struct MemoryMapTag {
    pub tag_type: u32,
    pub size: u32,
    pub entry_size: u32,
    pub entry_version: u32,
    // 条目跟随在这里
}

/// RSDP 标签（ACPI）
#[repr(C)]
pub struct RsdpTag {
    pub tag_type: u32,
    pub size: u32,
    pub rsdp: [u8; 0], // 变长
}

/// Multiboot 2 启动信息
pub struct Multiboot2BootInfo {
    info_ptr: *const Multiboot2Info,
}

impl Multiboot2BootInfo {
    /// 从 Multiboot 2 信息指针创建
    pub unsafe fn new(info_ptr: *const Multiboot2Info) -> Self {
        Self { info_ptr }
    }

    /// 遍历所有标签
    fn for_each_tag<F>(&self, mut f: F)
    where
        F: FnMut(&Tag),
    {
        unsafe {
            let info = &*self.info_ptr;
            let total_size = info.total_size as usize;
            let mut offset = 8; // 跳过 total_size 和 reserved

            while offset < total_size {
                let tag = &*((self.info_ptr as *const u8).add(offset) as *const Tag);
                
                if tag.tag_type == 0 {
                    break; // 结束标签
                }

                f(tag);

                // 对齐到 8 字节
                let size = ((tag.size + 7) / 8) * 8;
                offset += size as usize;
            }
        }
    }

    /// 获取特定类型的标签
    fn get_tag(&self, tag_type: TagType) -> Option<*const Tag> {
        let mut result = None;
        self.for_each_tag(|tag| {
            if tag.tag_type == tag_type as u32 {
                result = Some(tag as *const Tag);
            }
        });
        result
    }
}

impl BootInfo for Multiboot2BootInfo {
    fn framebuffer_info(&self) -> Option<FrameBufferInfo> {
        let tag_ptr = self.get_tag(TagType::FramebufferInfo)?;
        
        unsafe {
            let fb_tag = &*(tag_ptr as *const FramebufferTag);
            
            // 假设是 RGB 格式
            let pixel_format = PixelFormat::Rgb;
            let bytes_per_pixel = (fb_tag.bpp / 8) as usize;
            
            Some(FrameBufferInfo {
                width: fb_tag.width as usize,
                height: fb_tag.height as usize,
                stride: (fb_tag.pitch / bytes_per_pixel as u32) as usize,
                pixel_format,
                bytes_per_pixel,
                physical_address: fb_tag.addr as usize,
            })
        }
    }

    fn framebuffer_address(&self) -> Option<u64> {
        let tag_ptr = self.get_tag(TagType::FramebufferInfo)?;
        unsafe {
            let fb_tag = &*(tag_ptr as *const FramebufferTag);
            Some(fb_tag.addr)
        }
    }

    fn memory_regions(&self) -> &[MemoryRegion] {
        // TODO: 实现内存映射解析
        &[]
    }

    fn rsdp_address(&self) -> Option<u64> {
        // 尝试获取新的 ACPI RSDP
        if let Some(tag_ptr) = self.get_tag(TagType::AcpiNewRsdp) {
            unsafe {
                let rsdp_tag = &*(tag_ptr as *const RsdpTag);
                return Some(rsdp_tag.rsdp.as_ptr() as u64);
            }
        }
        
        // 回退到旧的 ACPI RSDP
        if let Some(tag_ptr) = self.get_tag(TagType::AcpiOldRsdp) {
            unsafe {
                let rsdp_tag = &*(tag_ptr as *const RsdpTag);
                return Some(rsdp_tag.rsdp.as_ptr() as u64);
            }
        }
        
        None
    }

    fn command_line(&self) -> Option<&str> {
        // TODO: 实现命令行解析
        None
    }
}

/// Multiboot 2 头 - 标记内核支持 Multiboot 2
#[used]
#[link_section = ".multiboot2"]
static MULTIBOOT2_HEADER: Multiboot2Header = Multiboot2Header {
    magic: MULTIBOOT2_MAGIC,
    architecture: MULTIBOOT2_ARCHITECTURE_I386,
    header_length: MULTIBOOT2_HEADER_LENGTH,
    checksum: (0u32.wrapping_sub(MULTIBOOT2_MAGIC)
        .wrapping_sub(MULTIBOOT2_ARCHITECTURE_I386)
        .wrapping_sub(MULTIBOOT2_HEADER_LENGTH)),
    end_tag: Multiboot2HeaderTag {
        tag_type: 0,
        flags: 0,
        size: 8,
    },
};

/// Multiboot 2 入口点
#[no_mangle]
pub extern "C" fn _start_multiboot2(info_ptr: *const Multiboot2Info, magic: u32) -> ! {
    // 验证魔数
    if magic != 0x36d76289 {
        panic!("Invalid Multiboot 2 magic number: {:#x}", magic);
    }

    // 初始化串口（用于早期调试）
    unsafe {
        crate::serial::init_serial_early();
    }

    // 解析 Multiboot 2 信息
    let boot_info = unsafe { Multiboot2BootInfo::new(info_ptr) };

    // 跳转到内核主函数
    crate::kernel_main_multiboot2(&boot_info);
}

/// Panic 处理程序
#[cfg(feature = "multiboot2")]
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
