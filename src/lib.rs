#![no_std]
mod log;
pub use log::*;
use xx_mutex_lock::{Mutex, OnceLock};
pub static CAN_PRINT: Mutex<usize> = Mutex::new(0);
pub static LOG: OnceLock<Log> = OnceLock::new();

#[macro_export]
macro_rules! info {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        if let Some(ref logger) = $crate::LOG.get() {
            if logger.level == $crate::Level::INFO && *(CAN_PRINT.lock()) == 0{
                logger.info(format_args!($fmt $(, $($arg)+)?));
            }
        }
    }
}

#[macro_export]
macro_rules! warn {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        if let Some(ref logger) = $crate::LOG.get()  {
            if logger.level != $crate::Level::ERR && *(CAN_PRINT.lock()) == 0{
                logger.warnning(format_args!($fmt $(, $($arg)+)?));
            }
        }
    }
}

#[macro_export]
macro_rules! error {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        if let Some(ref logger) = $crate::LOG.get() {
            if *(CAN_PRINT.lock()) == 0 {
                logger.error(format_args!($fmt $(, $($arg)+)?));
            }
        }
    }
}
pub fn init_log(writer: &'static dyn WriteLog, level: Level) {
    LOG.get_or_init(|| Log::init(writer, level));
}
pub struct LogGuard;

impl LogGuard {
    pub fn lock_log() -> LogGuard {
        let mut deep = CAN_PRINT.lock();
        *deep += 1;
        LogGuard
    }
}

impl Drop for LogGuard {
    fn drop(&mut self) {
        let mut deep = CAN_PRINT.lock();
        if *deep == 0 {
            panic!("info lock deep error ")
        }
        *deep -= 1;
    }
}

pub fn test_region() {
    #[cfg(no_log_inlog)]
    let a = LogGuard::lock_log();
    #[cfg(no_log_inlog)]
    let c = LogGuard::lock_log();
    info!("I am info {}", "Aa");
    #[cfg(no_log_inlog)]
    drop(c);
    warn!("I am warning {}", "Aa");
    #[cfg(no_log_inlog)]
    drop(a);
    error!("I am  error {}", "Aa");
}

#[cfg(test)]
mod tests {
    use crate::*;
    extern crate std;
    use core::fmt;
    use std::println;
    struct PT;

    impl WriteLog for PT {
        fn print(&self, log_content: fmt::Arguments) {
            println!("{}", log_content)
        }
    }

    #[test]
    fn tests() {
        #[cfg(no_log_inlog)]
        let a = LogGuard::lock_log();
        #[cfg(no_log_inlog)]
        let c = LogGuard::lock_log();
        static WRITER: PT = PT;
        init_log(&WRITER, Level::WARN);
        info!("I am info {}", "Aa");
        #[cfg(no_log_inlog)]
        drop(c);
        warn!("I am warning {}", "Aa");
        #[cfg(no_log_inlog)]
        drop(a);
        error!("I am  error {}", "Aa");
    }
}
