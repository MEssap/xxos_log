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

## Region 
可以给每个库设置一个标识符，然后用`cfg`的形式控制这个库的log是否打印出来。

一个简单的例子

```Rust
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
```
我们创建一个新的项目
```Shell
cargo new test && cd test
```
然后引用我们的库,在`./Cargo.toml`里面添加

```Shell
[dependencies]
xxos_log ={ path = "../xxos_log"}
```
然后在 `./.cargo/config.toml`中添加我们的cfg
```
[build]
rustflags = "--cfg no_log_inlog"
```

在`main.rs`中应用测试案例
```Rust
struct PT;
use core::fmt;
use xxos_log::{WriteLog, init_log, Level, test_region};
impl WriteLog for PT {
    fn print(&self, log_content: fmt::Arguments) {
        println!("{}", log_content)
    }
}


fn main() {
    static WRITER: PT = PT;
    init_log(&WRITER, Level::WARN);
    test_region();
}
```
`cargo run`运行得到以下结果
```
   Compiling test_log v0.1.0 (/home/light/baihai/xxos_all/test_log)
    Finished dev [unoptimized + debuginfo] target(s) in 0.10s
     Running `/home/light/baihai/xxos_all/test_log/target/debug/test_log`
logid	[ERROR]: /home/light/baihai/xxos_all/xxos_log/src/lib.rs:74 - I am  error Aa
```

