mod data;

use std::sync::Mutex;

use actix_web::{web::Data, App, HttpServer};
use data::Roulette;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let roulette_mutex: Data<Mutex<Vec<Roulette>>> = Data::new(Mutex::new(vec![]));

    HttpServer::new(move || App::new().app_data(roulette_mutex.clone()))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
