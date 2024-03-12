#![no_std]
pub use write_log::*;
pub use proc_macro_lib::Logger;
use xx_mutex_lock::OnceLock;

pub static LOG: OnceLock<Log> = OnceLock::new();
#[macro_export]
macro_rules! info {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        if let Some(ref logger) = $crate::LOG.get() {
            if logger.level == $crate::Level::INFO {
                logger.info(format_args!($fmt $(, $($arg)+)?));
            }
        }
    }
}

#[macro_export]
macro_rules! warn {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        if let Some(ref logger) = $crate::LOG.get() {
            if logger.level != $crate::Level::ERR {
                logger.warnning(format_args!($fmt $(, $($arg)+)?));
            }
        }
    }
}

#[macro_export]
macro_rules! error {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        if let Some(ref logger) = $crate::LOG.get() {
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
    use std::println;
    use proc_macro_lib::Logger;
    #[derive(Logger)]
    struct PT;
    #[test]
    fn tests() {
        static WRITER: PT = PT;
        init_log(&WRITER,Level::WARN);
        info!("I am info {}", "Aa");
        warn!("I am warning {}", "Aa");
        error!("I am  error {}", "Aa");
    }
}
