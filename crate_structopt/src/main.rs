use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Opt {
    // short和long不填写任何值表示使用默认的，该条在执行命令时传参就可以通过-d或者-debug实现
    #[structopt(short, long)]
    debug: bool,

    // 该条表示使用指定的-v或者-velocity来进行穿参数，如果没有指定参数那就使用默认值42
    #[structopt(short = "v", long = "velocity", default_value = "42")]
    speed: f64,

    // 传入路径参数，但是由于系统不同可能分隔符不一样为了统一解决路径就使用PathBuf类型
    // 当用户传入字符串会被自动解析成PathBuf，由于该字段没有指定short和long故只能通过位置进行传值
    #[structopt(parse(from_os_str))]
    input: PathBuf,

    
    // 该字段与上面类似
    #[structopt(parse(from_os_str))]
    output: Option<PathBuf>,

    // 用-o进行传值
    #[structopt(short)]
    out_type: String,

    // 该字段表示如果上面字段"out-type"是"file"那么必须给该字段传值，否则报错
    // 如果"out-type"不是"file"也可以按照需要手动给该字段传值或者忽略该字段
    #[structopt(name = "FILE", required_if("out-type", "file"))]
    file_name: Option<String>,
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
}