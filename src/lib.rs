pub trait WriteLog {
    fn print(log_content: &str);
}

#[macro_export]
macro_rules! log {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        // TODO: use WriteLog
        //$crate::console::stdout::print(format_args!($fmt $(, $($arg)+)?));
    };
}

#[cfg(test)]
mod tests {}
