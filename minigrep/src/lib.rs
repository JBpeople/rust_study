use std::{env, fs, process, error::Error};

pub struct Config {
    query: String,
    file_path: String,
}
impl Config {
    // 创建一个Config结构体
    pub fn build(args: &Vec<String>) -> Result<Config, &'static str>{
        if args.len() < 3 {
            return Err("输入参数数量少于3个！！！");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config {query, file_path})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // 读取文件数据绑定到变量上
    let content = fs::read_to_string(config.file_path)?;
    for line in search(&config.query, &content) {
        println!("{}", line);
    }
    Ok(())
}

// 使用生命周期标注是因为最后返回的是content变量里面的字符切片，让返回值保证和content活的一样久
fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            results.push(line)
        }
    }
    results
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "safe";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }
}