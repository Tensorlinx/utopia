use crate::{serial_println_safe, kernel_try};
use log::{Record, Level, Metadata, LevelFilter};
use crate::error::{KernelResult, KernelError};

static LOGGER: SimpleLogger = SimpleLogger;

pub fn init() -> KernelResult<()> {
    // 确保串口在使用日志框架之前被初始化
    kernel_try!(crate::serial::init_serial());
    
    // 初始化日志记录器
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Trace))
        .map_err(|_| KernelError::LoggerInitFailed)?;
    
    // 输出一条测试日志，确认日志系统正常工作
    log::debug!("Logger initialized successfully");
    Ok(())
}

struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            serial_println_safe!("[{}] {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}