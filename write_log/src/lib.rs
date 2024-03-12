#![no_std]
use core::fmt;

pub trait WriteLog: Sync {
    fn print(&self, log_content: core::fmt::Arguments);
}
#[derive(Debug, PartialEq, Eq)]
pub enum Level {
    INFO,
    WARN,
    ERR,
}

pub struct Log<'a> {
    pub writer: &'a dyn WriteLog,
    pub level: Level,
}

impl<'a> Log<'a> {
    pub fn init(writer: &'a dyn WriteLog, level: Level) -> Self {
        Self { writer, level }
    }

    #[track_caller]
    pub fn info(&self, s: fmt::Arguments) {
        let location = core::panic::Location::caller();

        let file_name = location.file();
        let file_line = location.line();

        let logid = "logid";

        self.writer.print(format_args!(
            "{}\t[\x1b[32mINFO\x1b[0m]:  {}:{} - {}",
            logid, file_name, file_line, s
        ));
    }

    #[track_caller]
    pub fn warnning(&self, s: fmt::Arguments) {
        let location = core::panic::Location::caller();
        let logid = "logid";
        self.writer.print(format_args!(
            "{}\t[\x1b[33mWARN\x1b[0m]:  {}:{} - {}",
            logid,
            location.file(),
            location.line(),
            s
        ));
    }

    #[track_caller]
    pub fn error(&self, s: fmt::Arguments) {
        let location = core::panic::Location::caller();
        let logid = "logid";
        self.writer.print(format_args!(
            "{}\t[\x1b[31mERROR\x1b[0m]: {}:{} - {}",
            logid,
            location.file(),
            location.line(),
            s
        ));
    }
}

