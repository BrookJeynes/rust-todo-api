#[macro_use] extern crate rocket;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub mod posts;
pub mod response_models;
pub mod models;
pub mod schema;

// Todo: use a connection pool (R2D2) as not to create a connection everytime it needs to be used
pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");

    PgConnection::establish(&database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![posts::list_posts, posts::list_post, posts::new_post, posts::publish_post, posts::delete_post])
}
