#![no_std]

use core::{fmt, panic::Location};

pub trait WriteLog {
    fn print(&self, log_content: fmt::Arguments);
}

pub struct Log<'a> {
    writer: &'a dyn WriteLog,
}

impl<'a> Log<'a> {
    fn init(writer: &'a dyn WriteLog) -> Self {
        Self { writer }
    }

    #[track_caller]
    fn info(&self, s: &str) {
        let location = Location::caller();

        let file_name = location.file();
        let file_line = location.line();

        let mut logid = "logid";

        self.writer.print(format_args!(
            "{}\t[\x1b[32mINFO\x1b[0m]\t{}:{}\t- {}",
            logid, file_name, file_line, s
        ));
    }

    #[track_caller]
    fn warnning(&self, s: &str) {
        let location = Location::caller();
        let mut logid = "logid";
        self.writer.print(format_args!(
            "{}\t[\x1b[33mWARN\x1b[0m]\t{}:{}\t- {}",
            logid,
            location.file(),
            location.line(),
            s
        ));
    }

    #[track_caller]
    fn error(&self, s: &str) {
        let location = Location::caller();
        let mut logid = "logid";
        self.writer.print(format_args!(
            "{}\t[\x1b[31mERROR\x1b[0m]\t{}:{}\t- {}",
            logid,
            location.file(),
            location.line(),
            s
        ));
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    extern crate std;
    use std::println;
    struct PT;

    impl WriteLog for PT {
        fn print(&self, log_content: fmt::Arguments) {
            println!("{}", log_content)
        }
    }

    #[test]
    fn tests() {
        let writer = PT;
        let log = Log::init(&writer);
        log.info("I am info !");
        log.warnning("I am warnning !");
        log.error("I am error !");
    }
}
