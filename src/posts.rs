use rocket::serde::json::Json;
use diesel::prelude::*;
use crate::models::{Post, NewPost};
use crate::response_models::{Response, ResponseBody};
use rocket::response::status::{NotFound, NoContent, Created};

#[get("/")]
pub fn list_posts() -> String {
    use crate::schema::posts;

    let posts: Vec<Post> = match posts::table.select(posts::all_columns).load::<Post>(&mut crate::establish_connection()) {
        Ok(posts) => posts,
        // doesn't seem like selecting everything will throw any errors, leaving room for specific error handling just in case though
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        }
    };

    let response = Response { body: ResponseBody::Posts(posts) };

    serde_json::to_string(&response).unwrap()
}

#[get("/post/<post_id>")]
pub fn list_post(post_id: i32) -> Result<String, NotFound<String>> {
    use crate::schema::posts;

    let post: Post = match posts::table.find(post_id).first::<Post>(&mut crate::establish_connection()) {
        Ok(post) => post,
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response { body: ResponseBody::Message(format!("Error selecting post with id {} - {}", post_id, err))};
                return Err(NotFound(serde_json::to_string(&response).unwrap()));
            },
            _ => {
                panic!("Database error - {}", err);
            }        
        }
    };

    let response = Response { body: ResponseBody::Post(post) };

    Ok(serde_json::to_string(&response).unwrap())
}

#[post("/new_post", format = "application/json", data = "<post>")]
pub fn new_post(post: Json<NewPost>) -> Created<String> {
    use crate::schema::posts;

    let post = post.into_inner();

    // refactor to return post as well
    match diesel::insert_into(posts::table).values(&post).get_result::<Post>(&mut crate::establish_connection()) {
        Ok(post) => {
            let response = Response { body: ResponseBody::Post(post) };
            Created::new("").tagged_body(serde_json::to_string(&response).unwrap())
        },
        // doesn't seem like insert_into() will throw any errors, leaving room for specific error handling just in case though
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }
}

#[get("/publish/<post_id>")]
pub fn publish_post(post_id: i32) -> Result<String, NotFound<String>> {
    use crate::schema::posts::dsl::*;

    let response: Response;

    let post = match diesel::update(posts.find(post_id)).set(published.eq(true)).get_result::<Post>(&mut crate::establish_connection()) {
        Ok(post) => post,
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response { body: ResponseBody::Message(format!("Error publishing post with id {} - {}", post_id, err))};
                return Err(NotFound(serde_json::to_string(&response).unwrap()));
            },
            _ => {
                panic!("Database error - {}", err);
            }        
        }
    };

    response = Response { body: ResponseBody::Post(post) };

    Ok(serde_json::to_string(&response).unwrap())
}

#[get("/delete/<post_id>")]
pub fn delete_post(post_id: i32) -> Result<NoContent, NotFound<String>> {
    use crate::schema::posts::dsl::*;

    let response: Response;

    let num_deleted = match diesel::delete(posts.filter(id.eq(post_id))).execute(&mut crate::establish_connection()) {
        Ok(count) => count,
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response { body: ResponseBody::Message(format!("Error publishing post with id {} - {}", post_id, err))};
                return Err(NotFound(serde_json::to_string(&response).unwrap()));
            },
            _ => {
                panic!("Database error - {}", err);
            }        
        }
    };

    if num_deleted > 0 {
        Ok(NoContent)
    } else {
        response = Response { body: ResponseBody::Message(format!("Error - no post with id {}", post_id))};
        Err(NotFound(serde_json::to_string(&response).unwrap()))
    } 
}
