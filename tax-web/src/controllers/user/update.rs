use actix_web::{HttpResponse, Responder, patch};

#[patch("/user")]
async fn handler() -> impl Responder {
    HttpResponse::Ok().body("Update user")
}
