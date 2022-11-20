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
pub struct PostsResponse {
    pub error: bool,
    pub data: Vec<Post>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct PostResponse {
    pub error: bool,
    pub data: Post,
}
