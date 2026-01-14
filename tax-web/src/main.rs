use actix_web::{
    App, HttpServer,
    web::{self},
};
use std::sync::{Arc, Mutex};

use crate::controllers::user;

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
        INSERT INTO users (firstName, lastName, age) VALUES ('Portgas', 'Ace', 42);
    ";
    connection.execute(query).unwrap();

    let db_connection = Arc::new(Mutex::new(connection));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_connection.clone()))
            .service(web::scope("/api").configure(user::routes::config))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
