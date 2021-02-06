use actix_web::{HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct IndexJsonRes {
    title: String,
}

pub async fn index() -> impl Responder {
    HttpResponse::Ok().json(IndexJsonRes {
        title: "Hello!".to_string(),
    })
}
