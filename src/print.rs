use core::{fmt, fmt::Write};

use crate::UART_INST;
//--------------------------------------------------------------------------------------------------
// Public Code
//--------------------------------------------------------------------------------------------------

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    unsafe { UART_INST.write_fmt(args).unwrap() };
}

/// Prints with a newline.
///
/// Carbon copy from <https://doc.rust-lang.org/src/std/macros.rs.html>
#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => ({
        $crate::print::_print(format_args!($($arg)*));
    })
}