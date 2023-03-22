use std::{path::PathBuf, error::Error};
use structopt::StructOpt;
use serde::Deserialize;

fn main() -> Result<(), Box<dyn Error>> {
    // 定义一个结构选项用以获取命令行参数
    #[derive(Debug, StructOpt)]
    struct Opt {
        #[structopt(short, long, parse(from_os_str))]
        input: PathBuf,
    }
    //定义结构体用以获取toml数据，注意结构体必须满足反序列化属性
    #[derive(Debug, Deserialize)]
    struct Config {
        input: Input,
    }

    #[derive(Debug, Deserialize)]
    struct Input {
        xml_file: PathBuf,
        json_file: PathBuf,
    }
    // 使用变量遮蔽用以获取结构选项中的参数
    let Opt = Opt::from_args();
    // 读取文件内容，转换成字符串
    let content = std::fs::read_to_string(&Opt.input)?;
    // 将字符串转换成定义的结构体格式
    let config: Config = toml::from_str(&content)?;

    println!("{:#?}", config);
    Ok(())
}
