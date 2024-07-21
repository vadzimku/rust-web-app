use serde::{Deserialize, Serialize};

pub struct User {
    pub id: u64,
    pub username: String,
}

#[derive(Deserialize)]
pub struct CreateUserRq {
    pub username: String,
}

#[derive(Serialize)]
pub struct CreateUserRs {
    pub id: u64,
    pub username: String,
}

#[derive(Serialize)]
pub struct GetUserRs {
    pub id: u64,
    pub username: String,
}
