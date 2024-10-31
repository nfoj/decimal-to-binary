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

- .env: Environment variable

    Obs: Dynamic values residing on your machine but outside your application. Sensitive data of your application!
    Examples: API keys, access tokens, passwords, connections urls, environment settings. 

```
    //.env
    USER=admin
    PASS=admin

    PORT=8080
    DATABASE_URL="postgres:..."


    // dotenv
    [dependencies]
    dotenv = "0.15.0"


    //
    use dotenv::dotenv;

    // main
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
      .bind(&address)? // add address 
      .run()
      .await
  }



          
```

