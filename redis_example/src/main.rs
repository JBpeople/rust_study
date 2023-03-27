use redis::Commands;

fn main() -> redis::RedisResult<()> {
    // 创建redis连接
    let client = redis::Client::open("redis://localhost/")?;
    let mut conn = client.get_connection()?;
    // 添加键值对
    conn.set("name", "Ken")?;
    conn.set("age", 23)?;
    conn.set("gender", "male")?;
    // 输出键值对
    println!("{}, {}, {}", 
    conn.get::<_, String>("name")?,
    conn.get::<_, String>("gender")?,
    conn.get::<_, i32>("age")?,
    );
    Ok(())
}
