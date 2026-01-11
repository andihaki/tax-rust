use std::sync::{Arc, Mutex};

use actix_web::{HttpResponse, Responder, get, web};
use sqlite::Connection;

#[get("/users")]
async fn handler(db: web::Data<Arc<Mutex<Connection>>>) -> impl Responder {
    print!("get user");
    let query = "SELECT * from users";
    let db = db.as_ref().lock().unwrap();
    let mut results = String::new();
    let result = db.iterate(query, |pairs| {
        for &(column, value) in pairs.iter() {
            // print!("{} = {}", name, value.unwrap())
            results.push_str(&format!("{}: {}\n", column, value.unwrap_or_default()));
        }
        results.push_str("\n");
        true
    });

    match result {
        Ok(_) => HttpResponse::Ok().body(results),
        Err(e) => HttpResponse::InternalServerError().body(format!("Database Error: {}", e)),
    }
}
