# Rust - Actix

- Small study to better understand actix.

```
// Cargo.tom
[dependencies]
actix-web = "4"


// src/main
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

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
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}


// Test
curl -i http://127.0.0.1:8080/
  
```

- Start server: println("Starting Server!");
- Shows the number of processor threads: println!("Starting thread!"); 

```
    async fn main() -> std::io::Result<()> {
      println!("Starting Server!"); // Start
      HttpServer::new(|| {
          println!("Starting thread!"); // Shows the number of processor threads.
          App::new()
    
```
