use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
}
