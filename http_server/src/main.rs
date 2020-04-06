use actix_web::{middleware, dev::BodyEncoding, HttpResponse, Responder, HttpServer, App, web, http};
use actix_cors::Cors;


async fn index() -> impl Responder {
    format!("Hello world")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().wrap(Cors::new()
        .allowed_origin("*")
        .allowed_methods(vec!["*"])
        .allowed_headers(vec!["*"])
        .allowed_header("*")
        .max_age(3600)
        .finish()).service(
        web::resource("/ready").to(index)))
        .bind("127.0.0.1:34578")?
        .run()
        .await
}
