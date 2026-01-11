use actix_web::{App, HttpServer, web};

mod controllers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    unsafe {
        std::env::set_var("RUST_LOG", "debug");
    }
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // .service(controllers::api::handler())
            .service(
                web::scope("/api")
                    .service(controllers::user::create::handler)
                    .service(controllers::user::delete::handler)
                    .service(controllers::user::detail::handler)
                    .service(controllers::user::update::handler),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
