use uart_16550::SerialPort;
use spin::Mutex;
use crate::constants::serial::*;
use crate::error::{KernelResult, KernelError, SafeWrite};
use core::fmt;

pub static SERIAL1: Mutex<Option<SerialPort>> = Mutex::new(None);

pub fn init_serial() -> KernelResult<()> {
    let mut serial_port = unsafe { SerialPort::new(COM1_BASE) };
    serial_port.init();
    *SERIAL1.lock() = Some(serial_port);
    Ok(())
}

#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) -> KernelResult<()> {
    use core::fmt::Write;
    if let Some(ref mut serial) = *SERIAL1.lock() {
        serial.write_fmt(args).map_err(|_| KernelError::WriteFailed)?;
        Ok(())
    } else {
        Err(KernelError::SerialInitFailed)
    }
}

/// 安全的串口打印函数（不会panic）
#[doc(hidden)]
pub fn _print_safe(args: ::core::fmt::Arguments) {
    use core::fmt::Write;
    if let Some(ref mut serial) = *SERIAL1.lock() {
        let _ = serial.write_fmt(args); // 忽略错误，避免panic
    }
}

/// Prints to the host through the serial interface.
#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        let _ = $crate::serial::_print(format_args!($($arg)*));
    };
}

/// Prints to the host through the serial interface, appending a newline.
#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(
        concat!($fmt, "\n"), $($arg)*));
}

/// 安全的串口打印宏（不会panic）
#[macro_export]
macro_rules! serial_print_safe {
    ($($arg:tt)*) => {
        $crate::serial::_print_safe(format_args!($($arg)*));
    };
}

/// 安全的串口打印宏，带换行（不会panic）
#[macro_export]
macro_rules! serial_println_safe {
    () => ($crate::serial_print_safe!("\n"));
    ($fmt:expr) => ($crate::serial_print_safe!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print_safe!(
        concat!($fmt, "\n"), $($arg)*));
}