use actix_web::{Responder, dev::HttpServiceFactory, get, web};

pub fn user_routes() -> impl HttpServiceFactory {
    web::scope("/user").service(index)
}

#[get("/")]
async fn index() -> impl Responder {
    "Hello World"
}
