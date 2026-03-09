//! 内核错误处理模块
//! 定义内核专用的错误类型和处理机制

use core::fmt;

/// 内核错误类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KernelError {
    /// VGA 初始化失败
    VgaInitFailed,
    /// 日志系统初始化失败
    LoggerInitFailed,
    /// 写入操作失败
    WriteFailed,
    /// 无效参数
    InvalidParameter,
    /// 硬件错误
    HardwareError,
}

impl fmt::Display for KernelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KernelError::VgaInitFailed => write!(f, "VGA initialization failed"),
            KernelError::LoggerInitFailed => write!(f, "Logger initialization failed"),
            KernelError::WriteFailed => write!(f, "Write operation failed"),
            KernelError::InvalidParameter => write!(f, "Invalid parameter"),
            KernelError::HardwareError => write!(f, "Hardware error"),
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

/// 实现 fmt::Error 到 KernelError 的转换
impl From<fmt::Error> for KernelError {
    fn from(_: fmt::Error) -> Self {
        KernelError::WriteFailed
    }
}