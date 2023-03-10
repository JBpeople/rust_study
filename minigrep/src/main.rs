use std::{env, process};
use minigrep::Config;
use minigrep::run;

fn main() {
    // 使用collect方法通常需要指定变量类型
    let args:Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err|{
        eprintln!("Application error: {err}");
        process::exit(1)
    });
    // 这里使用if let是因为我们并不关心函数正确时候的输出
    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1)
    }
}
