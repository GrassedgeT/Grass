use core::fmt::{self, Write};

struct StdOut;

impl Write for StdOut {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.bytes() {
            sbi_rt::console_write_byte(c);
        }
        Ok(())
    }
}

pub fn print(args: fmt::Arguments) {
    StdOut.write_fmt(args).unwrap();
}

/// print string macro
#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    }
}

/// println string macro
#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}
