mod router;

use actix_web::{HttpServer, App};
use std::io::Result;
use crate::router::{AppStateWithCounter, index};
use actix_web::web::{Data, get};
use std::sync::Mutex;

#[actix_web::main]
async fn main() -> Result<()> {
    let counter = Data::new(AppStateWithCounter {
        counter: Mutex::new(0)
    });
    HttpServer::new(move || {
        App::new()
            .app_data(counter.clone())
            .route("/", get().to(index))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
