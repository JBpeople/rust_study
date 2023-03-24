use std::{fs, path::PathBuf, error::Error};
use serde_json::Value;
use structopt::StructOpt;


fn main() -> Result<(), Box<dyn Error>>  {
    // 建立结构体获取命令行参数
    #[derive(Debug, StructOpt)]
    struct Opt {
        #[structopt(short, long, parse(from_os_str))]
        input: PathBuf,
    }
    // 获取结构体参数
    let Opt = Opt::from_args();
    // 读取json文件转换成字符串
    let json_str = fs::read_to_string(&Opt.input).unwrap();
    // 将字符串转换成json格式数据
    let json: Value = serde_json::from_str(&json_str)?;

    // 输出其中的id字段
    println!("{}", json["id"]);
    Ok(())
}
