//! 内核常量定义模块
//! 集中管理所有魔数和配置常量

/// 串口通信相关常量
pub mod serial {
    /// COM1 串口基地址
    pub const COM1_BASE: u16 = 0x3F8;
}

/// VGA 显示相关常量
pub mod vga {
    /// 字符宽度（像素）
    pub const CHAR_WIDTH: usize = 8;
    /// 字符高度（像素）
    pub const CHAR_HEIGHT: usize = 16;
    /// 前景色（白色）
    pub const FOREGROUND_COLOR: u32 = 0x00FFFFFF;
    /// 背景色（黑色）
    pub const BACKGROUND_COLOR: u32 = 0x00000000;
}

/// QEMU 退出码
pub mod qemu {
    /// 测试成功退出码
    pub const EXIT_SUCCESS: u32 = 0x10;
    /// 测试失败退出码
    pub const EXIT_FAILED: u32 = 0x11;
    /// QEMU 退出端口
    pub const EXIT_PORT: u16 = 0xf4;
}

/// 字体相关常量
pub mod font {
    /// ASCII 可打印字符起始值
    pub const ASCII_PRINTABLE_START: usize = 32;
    /// ASCII 可打印字符结束值
    pub const ASCII_PRINTABLE_END: usize = 127;
    /// 字体数据中的字符数量
    pub const FONT_CHAR_COUNT: usize = 96;
}