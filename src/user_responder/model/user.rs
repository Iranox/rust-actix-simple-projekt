use serde::Serialize;

#[derive(Serialize)]
pub struct User {
    pub username: String,
    pub email: String,
}
