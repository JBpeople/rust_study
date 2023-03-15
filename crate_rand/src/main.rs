use std::ops::Range;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    // 书籍给的方法是rng.gen_range(Range{1,100})该方法在现行rand库中会出现报错
    // 查看源码是一个满足SampleRange<T>特性的泛型变量
    // 查看该特性定义位置可以看到std::ops::Range结构体具有该特性，因此改成下列代码
    println!("{}", rng.gen_range(Range{start:1, end:100}));
    // 返回一个0-1的浮点数
    println!("{}", rng.gen::<f64>());
    // 返回一个布尔变量根据变量确定后面取值
    println!("{}", if rng.gen() { "Heads" } else { "Tails" });
}
