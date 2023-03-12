use std::{fs::{File, self}, io, path::Path};
use xml::reader::{EventReader, XmlEvent};

fn read_entire_xml_file<P: AsRef<Path>>(file_path: P) -> io::Result<String>{
    let mut content = String::new();
    let file = File::open(file_path)?;
    let er = EventReader::new(file);
    for event in er.into_iter() {
        if let XmlEvent::Characters(text) = event.expect("ToDo") {
            content.push_str(&text);
        }
    };
    Ok(content)
}

fn main() -> io::Result<()> {
    let file_path = r"E:\学习资料\CODE\rust_study\seroost\resource";
    let dir = fs::read_dir(file_path)?;
    for file in dir {
        let file_path = file?.path();
        let content = read_entire_xml_file(&file_path)?;
        println!("{file_path:?} => {length}", length = content.len());
    }
    Ok(())
}
