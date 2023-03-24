use std::{path::PathBuf, fs, error::Error};
use structopt::StructOpt;
use serde_derive::{Deserialize, Serialize};
use serde_json;

// 创建获取参数结构体
#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(short, parse(from_os_str))]
    input: PathBuf,
    #[structopt(short, parse(from_os_str))]
    output: PathBuf,
}

// 创建转换json文件的数据格式
#[derive(Debug, Deserialize, Serialize)]
struct Json {
    name: String,
    age: i32,
    is_student: bool,
    grades: Grades,
    visits: Vec<String>,
    address: Address,
}

#[derive(Debug, Deserialize, Serialize)]
struct Grades {
    math: i32,
    english: i32,
    science: i32,
}

#[derive(Debug, Deserialize, Serialize)]
struct Address {
    street: String,
    city: String,
    state: String,
    zip: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    // 获取命令行参数
    let Opt = Opt::from_args();

    // 转换json文件
    let mut json = {
        let text = fs::read_to_string(Opt.input)?;
        serde_json::from_str::<Json>(&text)?
    };

    // 给年龄加上10
    json.age += 10;

    // 把数据写到输出的json文件中
    fs::write(Opt.output, serde_json::to_string_pretty(&json)?);
    Ok(())
}
