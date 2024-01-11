# A simple Log
目前是用于xxos的log实现，为了通用性和解耦写成了一个库。
## How to use 
在使用之前，需要先impl WriteLog trait,并创建相应结构体，这个结构体的生命周期得是`'static`。
 - Example
```Rust
impl WriteLog for PT {
    fn print(&self, log_content: fmt::Arguments) {
        println!("{}", log_content)
    }
}

fn main() {
    static WRITER: PT = PT;
    init_log(&WRITER);
    info!("I am info {}","Aa");
    warn!("I am warning {}","Aa");
    error!("I am  error {}","Aa");
}
```
它将会打印
```shell
logid   [INFO]: src/lib.rs:49 - I am info Aa
logid   [WARN]: src/lib.rs:50 - I am warning Aa
logid   [ERROR]: src/lib.rs:51 - I am  error Aa
```