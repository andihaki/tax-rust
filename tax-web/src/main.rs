use actix_web::{App, HttpServer, web};
use std::sync::{Arc, Mutex};

mod controllers;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    unsafe {
        std::env::set_var("RUST_LOG", "debug");
    }
    env_logger::init();
    let connection = sqlite::open(":memory:").unwrap();

    let query = "
        CREATE TABLE users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            firstName TEXT, 
            lastName TEXT, 
            age INTEGER);
        INSERT INTO users (firstName, lastName, age) VALUES ('John', 'Doe', 42);
    ";
    connection.execute(query).unwrap();

    let db_connection = Arc::new(Mutex::new(connection));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_connection.clone()))
            // .service(controllers::api::handler())
            .service(
                web::scope("/api")
                    .service(controllers::user::create::handler)
                    .service(controllers::user::delete::handler)
                    .service(controllers::user::detail::handler)
                    .service(controllers::user::list::handler)
                    .service(controllers::user::update::handler),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
