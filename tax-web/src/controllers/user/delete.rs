use actix_web::{HttpResponse, Responder, delete};

#[delete("/user")]
async fn handler() -> impl Responder {
    HttpResponse::Ok().body("Delete user")
}
