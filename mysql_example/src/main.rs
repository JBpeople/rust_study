use mysql::{*, prelude::Queryable};

// 创建结构体
#[derive(Debug)]
struct Product {
    id: usize,
    name: String,
    price: f64,
    description: String,
    date: String,
}

// 创建表格
fn crate_db() -> Result<PooledConn> {
    // 填写数据库信息
    let username = "root";
    let password = "root";
    let host = "localhost";
    let port = "3307";
    let database = "Rust2023";
    // 创建url连接
    let url = format!("mysql://{}:{}@{}:{}/{}", username, password, host, port, database);
    let pool = Pool::new(url.as_str())?;
    let mut conn = pool.get_conn()?;
    // 如果存在删除掉products表
    let _ = conn.query_drop("DROP TABLE IF EXISTS products")?;
    // 创建一个products表
    conn.query_drop( 
        r#"
        CREATE TABLE products (
            id INT UNSIGNED NOT NULL AUTO_INCREMENT,
            name VARCHAR(255) NOT NULL,
            price DOUBLE NOT NULL,
            description TEXT,
            date DATE,
            PRIMARY KEY (id)
        );"#)?;
    Ok(conn)
}

// 填充数据
fn populate_db(conn: &mut PooledConn) -> Result<()> {
    // 插入一行数据
    conn.exec_batch(
        r#"INSERT INTO products (name, price, description, date)
        VALUES (:name, :price, :description, :date)"#,
        &[params! {
            "name" => "orange",
            "price" => 31.21,
            "description" => "Good",
            "date" => "2023-02-12",
        }])?;
    Ok(())
}

// 查询输出表格内容
fn print_db(conn: &mut PooledConn) -> Result<()> {
    // 查询products表中所有数据组成Vec<Product>格式
    let mut stmt = conn.query_map(
        "SELECT * FROM products",
        |(id, name, price, description, date)| {
            Product{ id, name, price, description, date }
        })?;
    // 对于每个在列表中的结构体进行输出展示
    for product in &stmt {
        println!("{:?}", product);
    }
    Ok(())

}

fn main() -> Result<()> {
    let mut conn = crate_db()?;
    populate_db(&mut conn)?;
    print_db(&mut conn)?;
    Ok(())
}
