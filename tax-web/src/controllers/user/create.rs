use actix_web::{HttpResponse, Responder, post};

#[post("/user")]
async fn handler() -> impl Responder {
    HttpResponse::Ok().body("Post user")
}
