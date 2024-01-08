#![no_std]
pub trait WriteLog {
    fn print(&self,log_content: &str);
}


pub struct Log<'a> {
    writer: & 'a dyn WriteLog,
}

impl<'a> Log<'a> {
    fn init(writer: &'a dyn WriteLog) -> Self{
        Self {
            writer,
        }
    }
}

impl<'a> Log<'a> {
    fn info(&self,s: &str) {
        self.writer.print(s);
    }
}



#[cfg(test)]
mod tests {
    #[test]
    fn tests(){
        use crate::*;
        extern crate std;
        use std::println;
        struct PT;
        impl WriteLog for PT {
            fn print(&self,log_content: &str) {
                println!("{}",log_content)
            }
        }
        let writer = PT;
        let log = Log::init(&writer);
        log.info("I am info !")
    }

}
