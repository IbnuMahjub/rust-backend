use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct NewUser {
    pub name: String,
    pub email: String,
}
