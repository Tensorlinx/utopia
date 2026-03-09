//! 内核错误处理模块
//! 定义内核专用的错误类型和处理机制

use core::fmt;

/// 内核错误类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KernelError {
    /// VGA 初始化失败
    VgaInitFailed,
    /// 串口初始化失败
    SerialInitFailed,
    /// 日志系统初始化失败
    LoggerInitFailed,
    /// 帧缓冲区不可用
    FrameBufferUnavailable,
    /// 写入操作失败
    WriteFailed,
    /// 内存不足
    OutOfMemory,
    /// 无效参数
    InvalidParameter,
    /// 操作超时
    Timeout,
    /// 硬件错误
    HardwareError,
    /// 未知错误
    Unknown,
}

impl fmt::Display for KernelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KernelError::VgaInitFailed => write!(f, "VGA initialization failed"),
            KernelError::SerialInitFailed => write!(f, "Serial port initialization failed"),
            KernelError::LoggerInitFailed => write!(f, "Logger initialization failed"),
            KernelError::FrameBufferUnavailable => write!(f, "Frame buffer is not available"),
            KernelError::WriteFailed => write!(f, "Write operation failed"),
            KernelError::OutOfMemory => write!(f, "Out of memory"),
            KernelError::InvalidParameter => write!(f, "Invalid parameter"),
            KernelError::Timeout => write!(f, "Operation timed out"),
            KernelError::HardwareError => write!(f, "Hardware error"),
            KernelError::Unknown => write!(f, "Unknown error"),
        }
    }
}

/// 内核结果类型
pub type KernelResult<T> = Result<T, KernelError>;

/// 错误处理宏
/// 用于简化错误处理代码
#[macro_export]
macro_rules! kernel_try {
    ($expr:expr) => {
        match $expr {
            Ok(val) => val,
            Err(err) => {
                log::error!("Kernel error: {}", err);
                return Err(err);
            }
        }
    };
}

/// 安全的写入操作
/// 提供带错误处理的写入功能
pub trait SafeWrite {
    /// 安全写入字符串
    fn safe_write_str(&mut self, s: &str) -> KernelResult<()>;
    
    /// 安全写入格式化参数
    fn safe_write_fmt(&mut self, args: fmt::Arguments) -> KernelResult<()>;
}

/// 实现 fmt::Error 到 KernelError 的转换
impl From<fmt::Error> for KernelError {
    fn from(_: fmt::Error) -> Self {
        KernelError::WriteFailed
    }
}