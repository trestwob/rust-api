use actix_web::{App, HttpServer, Responder};
use std::io::Result;

#[actix_web::get("/main")]
async fn home() -> impl Responder {
    format!("Hello world")
}


#[actix_web::main]
async fn main() -> Result<()> {
    let port = 3000;
    println!("Starting server on port {}", port);
    HttpServer::new(|| App::new().service(home)).bind(("127.0.0.1", port))?.workers(2).run().await
}
