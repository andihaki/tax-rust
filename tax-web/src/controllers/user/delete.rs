use std::sync::{Arc, Mutex};

use actix_web::{HttpResponse, Responder, delete, web};
use sqlite::Connection;

#[delete("/user/{id}")]
async fn handler(path: web::Path<i32>, db: web::Data<Arc<Mutex<Connection>>>) -> impl Responder {
    let id = path.into_inner();

    let query = format!(
        "SELECT * FROM users
         WHERE id = {}",
        id
    );
    let db = db.lock().unwrap();
    let mut results = String::new();
    let _ = db.iterate(query, |pairs| {
        for &column in pairs.iter() {
            results.push_str(&column.0);
        }
        true
    });

    if results.is_empty() {
        return HttpResponse::Ok().body("No user found");
    }

    let query = format!(
        "DELETE from users
         WHERE id = {}",
        id
    );

    let result = db.execute(query);

    println!("result: {:?}", result);
    println!("result: {}", result.iter().len());

    match result {
        Ok(_) => HttpResponse::Ok().body("User Successfully Deleted"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Database Error: {}", e)),
    }
}
