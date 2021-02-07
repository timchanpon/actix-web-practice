use actix_web::{HttpResponse, Responder, web};
use serde::{Deserialize, Serialize};

use super::posts_models::Post;

#[derive(Deserialize, Serialize)]
struct FindByIdJsonRes {
    title: String,
    body: String,
    count_posts: u32,
}

pub async fn find_by_id(web::Path(id): web::Path<u64>) -> impl Responder {
    let post = Post::by_id(id);
    let count_posts = Post::all().len() as u32;

    HttpResponse::Ok().json(FindByIdJsonRes {
        title: post.title.to_string(),
        body: post.body.to_string(),
        count_posts: count_posts,
    })
}
