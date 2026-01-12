use std::sync::{Arc, Mutex};

use actix_web::{HttpResponse, Responder, get, web};
use sqlite::Connection;

use crate::models::user::User;

#[get("/users")]
async fn handler(db: web::Data<Arc<Mutex<Connection>>>) -> impl Responder {
    let query = "SELECT * from users";
    let db = db.as_ref().lock().unwrap();
    let mut users = Vec::<User>::new();
    let mut results = String::new();
    let result = db.iterate(query, |pairs| {
        let mut user = User {
            // first_name: format!("").into(),
            // last_name: format!("").into(),
            first_name: Some(format!("")),
            last_name: Some(format!("")),
            age: Some(0),
        };
        for &(column, value) in pairs.iter() {
            // print!("{} = {}", name, value.unwrap())
            results.push_str(&format!("{}: {}\n", column, value.unwrap_or_default()));
            // @todo: how to dynamic key 'column'?
            // user[column] = value.unwrap_or_default();
            match column {
                "firstName" => user.first_name = Some(value.unwrap_or_default().to_string()),
                "lastName" => user.last_name = Some(value.unwrap_or_default().to_string()),
                "age" => user.age = value.unwrap_or_default().parse().ok(),
                _ => {}
            }
        }
        users.push(user);
        results.push_str("\n");
        true
    });

    match result {
        // Ok(_) => HttpResponse::Ok().body(results),
        Ok(_) => HttpResponse::Ok().json(users),
        Err(e) => HttpResponse::InternalServerError().body(format!("Database Error: {}", e)),
    }
}
