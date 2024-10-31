use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let port = std::env::var("PORT").unwrap_or("8080".to_string());
    let address = format!("127.0.0.1:{}", port);

    println!("Starting Server!"); // Start
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(&address)?
    .run()
    .await
}
