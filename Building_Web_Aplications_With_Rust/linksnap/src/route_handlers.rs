use actix_web::{HttpRequest, HttpResponse, Responder};

pub async fn index(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("Welcome to Linksnap Api Server")
}
