use actix_web::{HttpResponse, Responder, web};

use super::posts_models::Post;

pub async fn find_by_id(web::Path(id): web::Path<u64>) -> impl Responder {
    let user = Post::by_id(id);

    HttpResponse::Ok().json(Post {
        id: user.id,
        title: user.title.to_string(),
        body: user.body.to_string(),
    })
}
