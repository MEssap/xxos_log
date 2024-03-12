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
    init_log(&WRITER,Level::INFO);
    info!("I am info {}", "Aa");
    warn!("I am warning {}", "Aa");
    error!("I am  error {}", "Aa");
}
```
也可以使用我们提供的过程宏
```rust
use xxos_log::{info, init_log,Logger,WriteLog};
#[derive(Logger)]
struct PT;
fn main() {
    init_log(&PT, xxos_log::Level::INFO);
    info!("hello");
    warn!("I am warning {}", "Aa");
    error!("I am  error {}", "Aa");
}
```
它将会打印
```shell
logid   [INFO]: src/lib.rs:49 - I am info Aa
logid   [WARN]: src/lib.rs:50 - I am warning Aa
logid   [ERROR]: src/lib.rs:51 - I am  error Aa
```
 - Example 2 
```Rust
impl WriteLog for PT {
    fn print(&self, log_content: fmt::Arguments) {
        println!("{}", log_content)
    }
}

fn main() {
    static WRITER: PT = PT;
    init_log(&WRITER,Level::WARN);
    info!("I am info {}", "Aa");
    warn!("I am warning {}", "Aa");
    error!("I am  error {}", "Aa");
}
```
它将会打印
```shell
logid   [WARN]:  src/lib.rs:60 - I am warning Aa
logid   [ERROR]: src/lib.rs:61 - I am  error Aa
```
## TODO
- [ ] 在log前打印唯一标识`logid`
