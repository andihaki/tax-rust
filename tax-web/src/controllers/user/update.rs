use std::sync::{Arc, Mutex};

use actix_web::{HttpResponse, Responder, patch, web};
use sqlite::Connection;

// @todo: send it using request body?
#[patch("/user/{id}/{firstName}/{lastName}/{age}")]
async fn handler(
    path: web::Path<(i32, String, String, i32)>,
    db: web::Data<Arc<Mutex<Connection>>>,
) -> impl Responder {
    let (id, first_name, last_name, age) = path.into_inner();

    let query = format!(
        "
        UPDATE users 
        SET firstName = {}, lastName = {}, age = {} 
        where id = {}",
        first_name, last_name, age, id
    );
    let db = db.as_ref().lock().unwrap();
    let result = db.execute(query);

    match result {
        Ok(_) => HttpResponse::Ok().body("Successfully updated"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Database Error: {}", e)),
    }
}
