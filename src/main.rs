mod api;
mod data;

use std::{collections::HashMap, sync::Mutex};

use actix_web::{web::Data, App, HttpServer};
use api::create::create;
use data::Roulette;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let roulette_mutex: Data<Mutex<HashMap<String, Roulette>>> =
        Data::new(Mutex::new(HashMap::new()));

    HttpServer::new(move || App::new().service(create).app_data(roulette_mutex.clone()))
        .bind(("0.0.0.0", 8180))?
        .run()
        .await
}
