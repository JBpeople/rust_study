use xml::reader::{XmlEvent, EventReader};

fn main() {
    // 打开文件
    let file = std::fs::File::open("./data/example.xml").unwrap();
    // 获取文件的事件集
    let events = EventReader::new(file);
    // 创建一个字符串用以存储内部数据
    let mut content = String::new();
    // 对于事件集中的每个事件
    for event in events {
        // 如果匹配到标签之间的文字，将文字添加到字符串中
        if let XmlEvent::Characters(text) = event.expect("TODO") {
            content.push_str(&text);
            content.push_str("\n");
        }
    }
    println!("{}", content);
}