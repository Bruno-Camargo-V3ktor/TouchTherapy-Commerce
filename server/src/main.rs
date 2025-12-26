use actix_web::{App, HttpServer, Responder, get, web};
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    HttpServer::new(move || {
        App::new().service(web::scope("/api").service(web::scope("/users").service(index)))
    })
    .bind("0.0.0.0:4321")?
    .run()
    .await?;

    Ok(())
}

#[get("/")]
async fn index() -> impl Responder {
    "Hello World"
}
