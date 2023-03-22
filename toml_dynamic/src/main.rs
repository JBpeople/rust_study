use toml;
use std::{fs, error::Error};

fn main() -> Result<(), Box<dyn Error>>{
    // 读取文件内容，生成字符串
    let toml = fs::read_to_string("./src/config.toml")?;
    // 对字符串进行序列化
    let toml = toml.parse::<toml::Value>()?;
    println!("{:#?}", toml);
    Ok(())
}
