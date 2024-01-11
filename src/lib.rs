#![no_std]
mod log;
pub use log::*;
use xx_mutex_lock::OnceLock;

pub static LOG: OnceLock<Log> = OnceLock::new();
#[macro_export]
macro_rules! info {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        if let Some(ref logger) = LOG.get() {
            if logger.level == $crate::Level::INFO {
                logger.info(format_args!($fmt $(, $($arg)+)?));
            }
        }
    }
}

#[macro_export]
macro_rules! warn {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        if let Some(ref logger) = LOG.get() {
            if logger.level != $crate::Level::ERR {
                logger.warnning(format_args!($fmt $(, $($arg)+)?));
            }
        }
    }
}

#[macro_export]
macro_rules! error {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        if let Some(ref logger) = LOG.get() {
            logger.error(format_args!($fmt $(, $($arg)+)?));
        }
    }
}
pub fn init_log(writer: &'static dyn WriteLog,level: Level) {
    LOG.get_or_init(|| Log::init(writer,level));
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
        static WRITER: PT = PT;
        init_log(&WRITER,Level::WARN);
        info!("I am info {}", "Aa");
        warn!("I am warning {}", "Aa");
        error!("I am  error {}", "Aa");
    }
}
