use actix_web::{App, HttpServer, web::scope};
use std::io;

mod routes;

#[tokio::main]
async fn main() -> io::Result<()> {
    HttpServer::new(move || App::new().service(scope("/api").service(routes::user_routes())))
        .bind("0.0.0.0:4321")?
        .run()
        .await?;

    Ok(())
}
