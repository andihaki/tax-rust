use std::sync::{Arc, Mutex};

use actix_web::{HttpResponse, Responder, get, web};
use sqlite::Connection;

#[get("/user/{id}")]
async fn handler(path: web::Path<i32>, db: web::Data<Arc<Mutex<Connection>>>) -> impl Responder {
    let user_id = path.into_inner();
    print!("get user: {}", user_id);
    let query = format!("SELECT * from users where id = {}", user_id);
    let db = db.as_ref().lock().unwrap();
    let mut results = String::new();
    let result = db.iterate(query, |pairs| {
        for &(column, value) in pairs.iter() {
            results.push_str(&format!("{}: {}\n", column, value.unwrap_or_default()));
        }
        results.push_str("\n");
        true
    });

    match result {
        Ok(_) => {
            if results.is_empty() {
                HttpResponse::Ok().body("No user found")
            } else {
                HttpResponse::Ok().body(results)
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Database Error: {}", e)),
    }
}
