use actix_web::{
    HttpResponse, Responder,
    dev::HttpServiceFactory,
    get,
    web::{self},
};

#[get("/user")]
async fn get_user() -> impl Responder {
    HttpResponse::Ok().body("Get user")
}

pub fn _handler() -> impl HttpServiceFactory {
    return web::scope("/api")
        // .route("/user", web::get().to(test_api))
        .service(get_user);
}
