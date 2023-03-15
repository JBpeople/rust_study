#[macro_use]
extern crate log;
use env_logger;

// Rust代码在执行完cargo build之后默认会在/target/debug/{项目名}生成一个Unix文件
// 然后运行RUST_LOG=debug ./target/debug/{项目名} 就可以输出代码中的对应日志
// 注意debug为最高级会输出全部内容，info输出非debug内容，warn输出warn和error，error只输出error内容
fn main() {
    // 初始化日志
    env_logger::init();
    // 四种记录类型
    info!("This is an info message");
    warn!("This is a warning message");
    error!("This is an error message");
    debug!("This is a debug message");
}
