use actix_web::{HttpResponse, Responder, get};

#[get("/user")]
async fn handler() -> impl Responder {
    HttpResponse::Ok().body("Get user")
}
