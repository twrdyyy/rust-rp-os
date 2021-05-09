use crate::{bsp, console};

use core::fmt;

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use console::interface::Write;

    bsp::console::console().write_fmt(args).unwrap();
}

/// Print without a newline
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::print::_print(format_args!($($arg)*)));
}

/// Print with a newline
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ({
        $crate::print::_print(format_args_nl!($($arg)*));
    })
}

/// Debug
#[macro_export]
macro_rules! debug {
    ($string:expr) => ({
        #[allow(unused_imports)]
        use crate::time::time_manager;
        

        $crate::print::_print(format_args_nl!(
            concat!($string, "TIME: {:>3}s {:03} ms"),
            time_manager().uptime().as_secs(),
            time_manager().uptime().subsec_micros()/1000,
        ));
    });
    
}
