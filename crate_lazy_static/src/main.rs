use lazy_static::lazy_static;
use std::collections::HashMap;
// 程序启动时需要加载一些资源，而这些资源需要较长的初始化时间，为了减少程序启动时间，将一些不是那么重要的资源进行延迟加载
// 使用lazy_static来定义全局变量，只有变量第一次被使用的时候才会进行进行初始化
// 使用 lazy_static 定义全局变量时，我们必须将变量类型标注为 static ref，并使用 * 操作符来获取其值
// 这是因为 lazy_static 在生成代码时会将变量封装成一个静态引用（&'static T），而不是直接提供其值。
lazy_static! {
    static ref DICTIONARY: HashMap<i32, &'static str> = {
        let mut m = HashMap::new();
        m.insert(11, "foo");
        m.insert(12, "bar");
        println!("Initialized");
        m
    };
}

fn main() {
    println!("Started");
    // 初始化变量，输出Initialized
    println!("{:?}", *DICTIONARY);
    // 不再对变量进行初始化
    println!("{:?}", *DICTIONARY);
}
