use sbi_rt::{system_reset, NoReason, Shutdown, SystemFailure};

pub fn console_write_char(c: usize) {
    #[allow(deprecated)]
    sbi_rt::legacy::console_putchar(c);
}

// pub fn console_read_char() -> usize {
//     sbi_rt::console_read(bytes)
// }

pub fn shutdown(failure: bool) -> ! {
    if !failure {
        system_reset(Shutdown,NoReason);
    } else {
        system_reset(Shutdown,SystemFailure);
    }
    unreachable!()
}