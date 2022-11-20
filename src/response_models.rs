use rocket::serde::Serialize;
use crate::models::Post;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Response {
    pub error: bool,
    pub message: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct TodoResponse {
    pub error: bool,
    pub data: Vec<Post>,
}
