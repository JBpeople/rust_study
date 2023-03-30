use std::io::Write;
use actix_web::{web, App, HttpResponse, HttpServer, Responder, HttpRequest};

// 强制刷新标准输出流中的缓冲区，以确保所有数据都已经被写入标准输出设备中
fn flush_stdout() {
    std::io::stdout().flush().unwrap();
}

// 执行DELETE请求时运行该方法
async fn delete_file(info: web::Path<(String,)>) -> impl Responder {
    let filename = &info.0;
    print!("Deleting file \"{}\" ... ", filename);
    flush_stdout();

    // TODO: Delete the file.

    println!("Deleted file \"{}\"", filename);
    HttpResponse::Ok()
}

// 执行GET请求时运行该方法
async fn download_file(info: web::Path<(String,)>) -> impl Responder {
    let filename = &info.0;
    print!("Downloading file \"{}\" ... ", filename);
    flush_stdout();

    // TODO: Read the contents of the file.
    let contents = "Contents of the file.\n".to_string();

    println!("Downloaded file \"{}\"", filename);
    HttpResponse::Ok().content_type("text/plain").body(contents)
}

// 执行PUT请求时运行该方法
async fn upload_specified_file(info: web::Path<(String,)>) -> impl Responder {
    let filename = &info.0;
    print!("Uploading file \"{}\" ... ", filename);
    flush_stdout();

    // TODO: Get from the client the contents to write into the file.
    let _contents = "Contents of the file.\n".to_string();

    // TODO: Create the file and write the contents into it.

    println!("Uploaded file \"{}\"", filename);
    HttpResponse::Ok()
}

// 执行POST请求时运行该方法
async fn upload_new_file(info: web::Path<(String,)>) -> impl Responder {
    let filename = &info.0;
    print!("Uploading file \"{}*.txt\" ... ", filename);
    flush_stdout();

    // TODO: Get from the client the contents to write into the file.
    let _contents = "Contents of the file.\n".to_string();

    // TODO: Generate new filename and create that file.
    let file_id = 17;

    let filename = format!("{}{}.txt", filename, file_id);

    // TODO: Write the contents into the file.

    println!("Uploaded file \"{}\"", filename);
    HttpResponse::Ok().content_type("text/plain").body(filename)
}

async fn invalid_resource(req: HttpRequest) -> impl Responder {
    println!("Invalid URI: \"{}\"", req.uri());
    HttpResponse::NotFound()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server_address = "127.0.0.1:8080";
    println!("Listening at address {} ...", server_address);
    flush_stdout();
    // 创建服务器实例
    HttpServer::new(|| {
	    // 创建一个新的Web应用程序
        App::new()
            .route("/{filename}", web::delete().to(delete_file))
            .route("/{filename}", web::get().to(download_file))
            .route("/{filename}", web::put().to(upload_specified_file))
            .route("/{filename}", web::post().to(upload_new_file))
            .default_service(web::route().to(invalid_resource))
    })
    // 将服务器绑定到一个IP地址和端口上
    .bind(server_address)?
    // 开启站点的监听模式等待连接
    .run()
    .await
}