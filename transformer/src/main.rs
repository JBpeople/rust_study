use std::{path::PathBuf, error::Error, fs::{self, File}};
use toml::{self, Value};
use structopt::StructOpt;
use serde_json;
use serde_derive::Deserialize;
use mysql::{Pool, prelude::Queryable, params};
use xml::reader::{XmlEvent, EventReader};
use rusqlite::{Connection, Result};
use redis::{self, Commands};

// 获取命令行参数结构体
#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(short, long, parse(from_os_str))]
    input: PathBuf,
}

// 定义学生的信息数据结构体
#[derive(Debug, Deserialize)]
struct Student {
    name: String,
    age: i32,
    gender: String,
    major: String,
    class: String,
    score: Score,
}

#[derive(Debug, Deserialize)]
struct Score {
    math: i32,
    english: i32,
    programming: i32,
}

// 读取json文件中的学生信息，存储成一个Vec<Student>
fn recreate_json(json: &str) -> Result<Vec<Student>, Box<dyn Error>> {
    let data = fs::read_to_string(&json)?;
    let data = serde_json::from_str::<Vec<Student>>(&data)?;
    Ok(data)
}

// 将Vec<Student>存到mysql数据库中
fn populate_mysql(config: &Value, students: &Vec<Student>) -> Result<(), Box<dyn Error>> {
    // 获取数据库信息，创建url连接
    let mysql_config = &config["mysql_serve"];
    let host = mysql_config["host"].as_str().unwrap();
    let port = mysql_config["port"].as_str().unwrap();
    let username = mysql_config["username"].as_str().unwrap();
    let password = mysql_config["password"].as_str().unwrap();
    let database = mysql_config["database"].as_str().unwrap();
    let url = format!("mysql://{}:{}@{}:{}/{}", username, password, host, port, database);
    // 创建mysql连接
    let pool = Pool::new(url.as_str())?;
    let mut conn = pool.get_conn()?;
    // 如果存在删除students表格，创建students表格
    conn.query_drop("DROP TABLE IF EXISTS students")?;
    conn.query_drop(
        r#"CREATE TABLE students (
            name TEXT,
            age INTEGER,
            gender TEXT,
            major TEXT,
            class TEXT,
            math_score INTEGER,
            english_score INTEGER,
            programming_score INTEGER
        );"#)?;
    // 往创建的表格里面插入Vec<Student>数据
    conn.exec_batch(
        r"INSERT INTO students (name, age, gender, major, class, math_score, english_score, programming_score)
        VALUES (:name, :age, :gender, :major, :class, :math_score, :english_score, :programming_score)", 
        students.iter().map(|student| params! {
            "name" => student.name.as_str(),
            "age" => student.age,
            "gender" => student.gender.as_str(),
            "major" => student.major.as_str(),
            "class" => student.class.as_str(),
            "math_score" => student.score.math,
            "english_score" => student.score.english,
            "programming_score" => student.score.programming,
        }))?;
    // 返回程序运行成功
    Ok(())
}

// 读取xml文件返回信息
fn recreate_xml(xml: &str) -> Result<Vec<String>, Box<dyn Error>> {
    // 打开xml文件，读取内部事件
    let file = File::open(xml)?;
    let events = EventReader::new(file);
    // 创建一个Vec收集信息
    let mut data = Vec::new();
    // 遍历xml内部事件
    for event in events {
        match event {
            Ok(XmlEvent::Characters(text)) => data.push(text.to_owned()),
            _ => continue,
        }
    }

    Ok(data)
}

fn creat_db(students: &Vec<Student>, output: &str) -> Result<()> {
    let db_file = format!("{}/students.db", output);
    let conn = Connection::open(db_file)?;
    conn.execute("DROP TABLE IF EXISTS students", rusqlite::params![])?;
    conn.execute(
        r#"CREATE TABLE students (
            name TEXT,
            age INTEGER,
            gender TEXT,
            major TEXT,
            class TEXT,
            math_score INTEGER,
            english_score INTEGER,
            programming_score INTEGER
        );"#, rusqlite::params![])?;
    
    for student in students {
        conn.execute(
            "INSERT INTO students (name, age, gender, major, class)
            VALUES ($1, $2, $3, $4, $5);", 
            rusqlite::params![student.name, student.age, student.gender, student.major, student.class])?;
    };
    Ok(())
}

fn populate_redis(config: &Value, key: &str, val: i32) -> Result<(), Box<dyn Error>>{
    let redis = &config["redis_server"];
    let host = &redis["host"];
    let url = format!("redis://{}", host);
    let client = redis::Client::open(url)?;
    let mut conn = client.get_connection()?;
    conn.set(key, val)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // 获取命令行参数
    let opt = Opt::from_args();
    // 读取配置config文件
    let config = fs::read_to_string(opt.input)?;
    let config = config.parse::<toml::Value>()?;
    // 读取config文件中指定的json文件
    let json_file = config["input"]["json_file"].as_str().unwrap();
    let json_data = recreate_json(json_file)?;
    populate_mysql(&config, &json_data)?;
    // 读取xml文件，输出xml内部字段
    let xml_file = config["input"]["xml_file"].as_str().unwrap();
    let xml_data = recreate_xml(xml_file)?;
    println!("{:?}", xml_data);
    // 读取输出文件路径，传入方法创建一个本地db文件
    let output = config["output"]["file"].as_str().unwrap();
    creat_db(&json_data, output)?;
    // 给redis插入一个值
    populate_redis(&config, "age", 32)?;
    Ok(())
}
