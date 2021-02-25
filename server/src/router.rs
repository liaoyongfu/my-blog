use actix_web::{web, App, HttpServer};
use std::sync::Mutex;

pub struct AppStateWithCounter {
    pub(crate) counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
}

pub async fn index(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard
    *counter += 1; // <- access counter inside MutexGuard

    format!("Request number: {}", counter) // <- response with count
}
