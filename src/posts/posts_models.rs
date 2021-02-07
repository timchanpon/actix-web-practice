use diesel::prelude::*;

use crate::db::connection::establish_connection;
use crate::db::schema::posts;

#[derive(Debug, Queryable)]
pub struct Post {
    pub id: u64,
    pub title: String,
    pub body: String,
}

impl Post {
    pub fn all() -> Vec<Post> {
        let connection = establish_connection();

        posts::dsl::posts
            .load::<Post>(&connection)
            .expect("Post is not found.")
    }

    pub fn by_id(id: u64) -> Post {
        let connection = establish_connection();

        posts::dsl::posts
            .find(id)
            .first::<Post>(&connection)
            .expect("Post is not found.")
    }
}
