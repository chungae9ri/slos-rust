use crate::drivers::uart::uart::StdioUart;
use core::fmt;

//--------------------------------------------------------------------------------------------------
// Public Code
//--------------------------------------------------------------------------------------------------

#[doc(hidden)]
pub fn _print(uart:&mut StdioUart, args: fmt::Arguments) {
    uart.write_fmt(args).unwrap();
}

/// Prints with a newline.
///
/// Carbon copy from <https://doc.rust-lang.org/src/std/macros.rs.html>
#[macro_export]
macro_rules! println {
    ($uart:expr, $($arg:tt)*) => ({
        $crate::print::_print($uart, format_args!($($arg)*));
    })
}