/*

// Aplication no Import files

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct Input {
    number: u32,
}

async fn convert_to_binary(info: web::Json<Input>) -> impl Responder {
    let binary = format!("{:b}", info.number);
    HttpResponse::Ok().body(binary)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/convert", web::post().to(convert_to_binary)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

// test
// curl -X POST -H "Content-Type: application/json" -d '{"number": 10}' http://127.0.0.1:8080/convert

*/

// Modulo Import files + index
use actix_files::Files;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct Input {
    number: u32,
}

async fn convert_to_binary(info: web::Json<Input>) -> impl Responder {
    let binary = format!("{:b}", info.number);
    HttpResponse::Ok().body(binary)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/convert", web::post().to(convert_to_binary))
            .service(Files::new("/", "./static").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// test
// http://127.0.0.1:8080
