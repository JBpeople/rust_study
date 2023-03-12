use std::{fs::{File, self}, io, path::Path};
use xml::reader::{EventReader, XmlEvent};

// 读取全部的xml文件
fn read_entire_xml_file<P: AsRef<Path>>(file_path: P) -> io::Result<String>{
    // 定义一个可变字符用以收集后面匹配字符
    let mut content = String::new();
    // 打开文件
    let file = File::open(file_path)?;
    // 创建事件读取器读取每个文件
    let er = EventReader::new(file);
    // 对于每个事件都进行遍历
    for event in er.into_iter() {
        if let XmlEvent::Comment(text) = event.expect("ToDo") {
            content.push_str(&text);
        }
    };
    Ok(content)
}

fn main() -> io::Result<()> {
    // 指定文件路径
    let file_path = r"E:\学习资料\CODE\rust_study\search_xml\resource";
    // 读取文件夹
    let dir = fs::read_dir(file_path)?;
    // 遍历文件夹下面的每个文件
    for file in dir {
        let file_path = file?.path();
        let content = read_entire_xml_file(&file_path)?;
        println!("{file_path:?} => {length}", length = content.len());
    }
    Ok(())
}
