use actix_web::HttpResponse;

// impl Responder
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
