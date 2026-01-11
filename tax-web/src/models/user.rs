use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub age: Option<i32>,
}
