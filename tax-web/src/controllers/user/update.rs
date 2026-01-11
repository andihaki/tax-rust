use std::sync::{Arc, Mutex};

use actix_web::{HttpResponse, Responder, patch, web};
use serde::Deserialize;
use sqlite::Connection;

#[derive(Deserialize, Debug)]
struct User {
    first_name: Option<String>,
    last_name: Option<String>,
    age: Option<i32>,
}

#[patch("/user/{id}")]
async fn handler(
    path: web::Path<i32>,
    user: web::Json<User>,
    db: web::Data<Arc<Mutex<Connection>>>,
) -> impl Responder {
    let id = path.into_inner();
    // println!("{:?}", user);

    let first_name = match &user.first_name {
        Some(value) => format!("firstName = '{}',", value),
        None => format!(""),
    };
    let last_name = match &user.last_name {
        Some(value) => format!("lastName = '{}',", value),
        None => format!(""),
    };
    let age = match user.age {
        Some(age) => format!("age = {}", age),
        None => format!(""),
    };
    let query = format!(
        "UPDATE users 
         SET {} {} {} 
         WHERE id = {}",
        first_name, last_name, age, id
    );

    // println!("{}", query);

    let db = db.lock().unwrap();
    let result = db.execute(query);

    match result {
        Ok(_) => HttpResponse::Ok().body("Successfully updated"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Database Error: {}", e)),
    }
}
