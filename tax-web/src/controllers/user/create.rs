use std::sync::{Arc, Mutex};

use actix_web::{HttpResponse, Responder, post, web};
use sqlite::Connection;

use crate::models::user::User;

#[post("/user")]
async fn handler(user: web::Json<User>, db: web::Data<Arc<Mutex<Connection>>>) -> impl Responder {
    let first_name = match &user.first_name {
        Some(value) => value,
        None => return HttpResponse::BadRequest().body("first_name is required!"),
    };
    let last_name = match &user.last_name {
        Some(value) => value,
        None => return HttpResponse::BadRequest().body("last_name is required!"),
    };
    let age = match user.age {
        Some(value) => value,
        None => return HttpResponse::BadRequest().body("age is required!"),
    };

    let query = format!(
        "INSERT INTO users
         (firstName, lastName, age)
         VALUES ('{}', '{}', {})",
        first_name, last_name, age
    );
    let db = db.lock().unwrap();
    let result = db.execute(query);

    match result {
        Ok(_) => HttpResponse::Ok().body("Successfully add user"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Database Error: {}", e)),
    }
}
