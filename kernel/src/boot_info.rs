//! 启动信息抽象层
//! 支持多种引导加载程序（bootloader_api 和 limine）

use core::fmt;

/// 帧缓冲区信息
#[derive(Debug, Clone, Copy)]
pub struct FrameBufferInfo {
    pub width: usize,
    pub height: usize,
    pub stride: usize,
    pub pixel_format: PixelFormat,
    pub bytes_per_pixel: usize,
    pub physical_address: usize,
}

/// 像素格式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PixelFormat {
    Rgb,
    Bgr,
    U8,
    Unknown,
}

/// 内存区域类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryRegionType {
    Usable,
    Reserved,
    AcpiReclaimable,
    AcpiNvs,
    BadMemory,
    BootloaderReclaimable,
    KernelAndModules,
    Framebuffer,
}

/// 内存区域
#[derive(Debug, Clone, Copy)]
pub struct MemoryRegion {
    pub start: u64,
    pub end: u64,
    pub region_type: MemoryRegionType,
}

/// 启动信息 trait
/// 抽象不同引导加载程序的差异
pub trait BootInfo {
    /// 获取帧缓冲区信息
    fn framebuffer_info(&self) -> Option<FrameBufferInfo>;

    /// 获取帧缓冲区物理地址
    fn framebuffer_address(&self) -> Option<u64>;

    /// 获取内存映射迭代器
    fn memory_regions(&self) -> &[MemoryRegion];

    /// 获取 RSDP 地址（ACPI）
    fn rsdp_address(&self) -> Option<u64>;

    /// 获取命令行参数
    fn command_line(&self) -> Option<&str>;
}

/// 启动信息包装类型
pub enum BootInfoWrapper {
    #[cfg(feature = "bootloader_api")]
    BootloaderApi(&'static mut bootloader_api::BootInfo),
    #[cfg(feature = "limine")]
    Limine(&'static LimineBootInfo),
    #[cfg(feature = "multiboot2")]
    Multiboot2(&'static crate::multiboot2::Multiboot2BootInfo),
}

impl BootInfo for BootInfoWrapper {
    fn framebuffer_info(&self) -> Option<FrameBufferInfo> {
        match self {
            #[cfg(feature = "bootloader_api")]
            BootInfoWrapper::BootloaderApi(info) => {
                info.framebuffer.as_ref().map(|fb| {
                    let info = fb.info();
                    FrameBufferInfo {
                        width: info.width,
                        height: info.height,
                        stride: info.stride,
                        pixel_format: match info.pixel_format {
                            bootloader_api::info::PixelFormat::Rgb => PixelFormat::Rgb,
                            bootloader_api::info::PixelFormat::Bgr => PixelFormat::Bgr,
                            bootloader_api::info::PixelFormat::U8 => PixelFormat::U8,
                            _ => PixelFormat::Unknown,
                        },
                        bytes_per_pixel: info.bytes_per_pixel,
                        physical_address: fb.buffer().as_ptr() as usize,
                    }
                })
            }
            #[cfg(feature = "limine")]
            BootInfoWrapper::Limine(info) => info.framebuffer_info(),
            #[cfg(feature = "multiboot2")]
            BootInfoWrapper::Multiboot2(info) => info.framebuffer_info(),
        }
    }

    fn framebuffer_address(&self) -> Option<u64> {
        match self {
            #[cfg(feature = "bootloader_api")]
            BootInfoWrapper::BootloaderApi(info) => {
                info.framebuffer.as_ref().map(|fb| fb.buffer().as_ptr() as u64)
            }
            #[cfg(feature = "limine")]
            BootInfoWrapper::Limine(info) => info.framebuffer_address(),
            #[cfg(feature = "multiboot2")]
            BootInfoWrapper::Multiboot2(info) => info.framebuffer_address(),
        }
    }

    fn memory_regions(&self) -> &[MemoryRegion] {
        match self {
            #[cfg(feature = "bootloader_api")]
            BootInfoWrapper::BootloaderApi(_info) => {
                // 将 bootloader_api 的内存映射转换为通用格式
                // 这里简化处理，实际需要动态分配或静态数组
                &[]
            }
            #[cfg(feature = "limine")]
            BootInfoWrapper::Limine(info) => info.memory_regions(),
            #[cfg(feature = "multiboot2")]
            BootInfoWrapper::Multiboot2(info) => info.memory_regions(),
        }
    }

    fn rsdp_address(&self) -> Option<u64> {
        match self {
            #[cfg(feature = "bootloader_api")]
            BootInfoWrapper::BootloaderApi(info) => info.rsdp_addr.into_option(),
            #[cfg(feature = "limine")]
            BootInfoWrapper::Limine(info) => info.rsdp_address(),
            #[cfg(feature = "multiboot2")]
            BootInfoWrapper::Multiboot2(info) => info.rsdp_address(),
        }
    }

    fn command_line(&self) -> Option<&str> {
        match self {
            #[cfg(feature = "bootloader_api")]
            BootInfoWrapper::BootloaderApi(_) => None,
            #[cfg(feature = "limine")]
            BootInfoWrapper::Limine(info) => info.command_line(),
            #[cfg(feature = "multiboot2")]
            BootInfoWrapper::Multiboot2(info) => info.command_line(),
        }
    }
}

/// Limine 启动信息结构
#[cfg(feature = "limine")]
pub struct LimineBootInfo {
    pub framebuffer: Option<LimineFramebuffer>,
    pub memory_map: &'static [MemoryRegion],
    pub rsdp: Option<u64>,
    pub cmdline: Option<&'static str>,
}

#[cfg(feature = "limine")]
pub struct LimineFramebuffer {
    pub address: u64,
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
}

#[cfg(feature = "limine")]
impl LimineBootInfo {
    fn framebuffer_info(&self) -> Option<FrameBufferInfo> {
        self.framebuffer.as_ref().map(|fb| {
            let pixel_format = if fb.memory_model == 1 && fb.red_mask_size == 8 && fb.red_mask_shift == 16 {
                PixelFormat::Rgb
            } else if fb.memory_model == 1 && fb.red_mask_size == 8 && fb.red_mask_shift == 0 {
                PixelFormat::Bgr
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
        self.framebuffer.as_ref().map(|fb| fb.address)
    }

    fn memory_regions(&self) -> &[MemoryRegion] {
        self.memory_map
    }

    fn rsdp_address(&self) -> Option<u64> {
        self.rsdp
    }

    fn command_line(&self) -> Option<&str> {
        self.cmdline
    }
}
