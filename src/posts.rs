use rocket::serde::json::Json;
use diesel::prelude::*;
use crate::models::{Post, NewPost};
use crate::response_models::{Response, TodoResponse};

#[get("/")]
pub fn list() -> String {
    use crate::schema::posts;

    let posts: Vec<Post> = match posts::table.select(posts::all_columns).load::<Post>(&mut crate::establish_connection()) {
        Ok(posts) => posts,
        Err(err) => {
            let response = Response { error: true, message: format!("Error getting posts - {}", err) };
            return serde_json::to_string(&response).unwrap(); 
        } 
    };

    let response = TodoResponse { error: false, data: posts };

    serde_json::to_string(&response).unwrap()
}


#[post("/new_post", format = "application/json", data = "<post>")]
pub fn new_post(post: Json<NewPost>) -> String {
    use crate::schema::posts;

    let post = post.into_inner();

    match diesel::insert_into(posts::table).values(&post).execute(&mut crate::establish_connection()) {
        Ok(_) => {
            let response = Response {error: false, message: format!("Successfully inserted post {}", post.title)};
            serde_json::to_string(&response).unwrap()
        },
        Err(err) => {
            let response = Response {error: true, message: format!("Error saving post - {}", err)};
            serde_json::to_string(&response).unwrap()
        }
    }
}

#[get("/delete_post/<post_id>")]
pub fn delete_post(post_id: i32) -> String {
    use crate::schema::posts::dsl::*;

    let response: Response;

    let num_deleted = match diesel::delete(posts.filter(id.eq(post_id))).execute(&mut crate::establish_connection()) {
        Ok(count) => count,
        Err(err) => {
            response = Response { error: true, message: format!("Error deleting post with id {} - {}", post_id, err) };
            return serde_json::to_string(&response).unwrap(); 
        } 
    };

    if num_deleted > 0 {
        response = Response { error: false, message: format!("Successfully deleted post with id {}", post_id) };
    } else {
        response = Response { error: true, message: format!("Error - no post with id {}", post_id) };
    }

    serde_json::to_string(&response).unwrap()
}

#[get("/publish/<post_id>")]
pub fn publish_post(post_id: i32) -> String {
    use crate::schema::posts::dsl::*;

    let response: Response;

    let post = match diesel::update(posts.find(post_id)).set(published.eq(true)).get_result::<Post>(&mut crate::establish_connection()) {
        Ok(post) => post,
        Err(err) => {
            response = Response { error: true, message: format!("Error publishing post with id {} - {}", post_id, err) };
            return serde_json::to_string(&response).unwrap(); 
        } 
    };

    response = Response { error: false, message: format!("Successfully published post '{}'", post.title) };

    serde_json::to_string(&response).unwrap()
}
