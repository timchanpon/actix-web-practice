extern crate sample_app as app;

use actix_web::{App, HttpServer, web};
use std::io;

use app::posts::posts_views;
use app::utils::utils_views;

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(utils_views::index))
            .route("/favicon.ico", web::get().to(utils_views::favicon))
            .service(
                web::scope("/api")
                    .service(
                        web::scope("/posts")
                            .route("/{id}", web::get().to(posts_views::find_by_id))
                    )
            )
            .default_service(
                web::resource("")
                    .route(web::get().to(utils_views::not_found))
            )
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
