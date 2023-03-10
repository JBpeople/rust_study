use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    loop {
        println!("请输入你猜测的数字：");
        // 创建一个空的字符串用于存储后面的用户输入的数字
        let mut guess = String::new();
        // 创建一个1到100的随机数
        let secret_num = rand::thread_rng().gen_range(1..101);
        // 读取用户输入
        io::stdin().read_line(&mut guess).expect("无法读取输入！！！");
        // 把用户的输入转成u32数字
        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // 比较用户输入和生成随机数的大小
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("小了～"),
            Ordering::Greater => println!("大了～"),
            Ordering::Equal => {
                println!("恭喜你猜对了！！！");
                break;
            }
        }
    }
}
