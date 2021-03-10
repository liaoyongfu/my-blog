mod router;
mod config;

use actix_web::{HttpServer, App, web, HttpResponse, http};
use std::io::Result;
use actix_web::web::{Data, get};
use std::sync::Mutex;
use crate::router::get_articles;
use actix_cors::Cors;

#[actix_web::main]
async fn main() -> Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            // TODO 如何设置 *
            .allowed_origin("http://localhost:8888")
            // .allowed_methods(vec!["GET", "POST"])
            // .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            // .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors)
            .service(get_articles)
    })
        .bind("127.0.0.1:9999")?
        .run()
        .await
}
