use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::db::connection::establish_connection;
use crate::db::schema::posts;

#[derive(Debug, Deserialize, Queryable, Serialize)]
pub struct Post {
    pub id: u64,
    pub title: String,
    pub body: String,
}

impl Post {
    pub fn by_id(id: u64) -> Post {
        let connection = establish_connection();

        posts::dsl::posts
            .find(id)
            .first::<Post>(&connection)
            .expect("Post is not found.")
    }
}
