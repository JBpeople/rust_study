use rusqlite::{params, Connection, Result};

// 该结构体用于后面去获取数据库字段数据
#[derive(Debug)]
struct Student {
    name: String,
    age: i32,
    gender: String,
    hobby: String,
    score: i32,
}

// 创建一个DB返回连接
fn create_db() -> Result<Connection> {
    let database_file = "students.db";
    let conn = Connection::open(&database_file)?;
    // 执行SQL语句如果存在Student表格则删除该表格
    let _ = conn.execute("DROP TABLE IF EXISTS Student", params![]);
    // 执行SQL语句建表
    conn.execute(
        "CREATE TABLE Student (
            name VARCHAR(255),
            age INTEGER,
            gender VARCHAR(255),
            hobby VARCHAR(255),
            score INTEGER);", 
        params![])?;
    Ok(conn)
}

// 填充数据库
fn populate_db(conn: &Connection) -> Result<()> {
    // 执行SQL语句插值
    conn.execute(
        "INSERT INTO Student (name, age, gender, hobby, score)
        VALUES ($1, $2, $3, $4, $5);",
        params!["John", 25, "Male", "Reading", 90])?;
    conn.execute(
        "INSERT INTO Student (name, age, gender, hobby, score)
        VALUES ($1, $2, $3, $4, $5);",
        params!["Ann", 25, "Male", "Reading", 78])?;
    Ok(())
}

// 打印数据库内容
fn print_db(conn: &Connection) -> Result<()> {
    // 获取SQL语句查询出来的数据
    let mut command = conn.prepare(
        "SELECT * FROM Student")?;
    // 遍历查询出来的每行数据，把每行数据转换Student结构体
    for student in command.query_map(params![], |row| {
        Ok(Student {
            name: row.get(0)?,
            age: row.get(1)?,
            gender: row.get(2)?,
            hobby: row.get(3)?,
            score: row.get(4)?,
        })
    })? {
        // 如果遍历出来的值是Student结构体，打印一句话
        if let Ok(item) = student {
            println!("{}得分{}", item.name, item.score);
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let conn = create_db()?;
    populate_db(&conn)?;
    print_db(&conn)?;
    Ok(())
}
