mod api;
mod data;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use actix_web::{web::Data, App, HttpServer};
use api::{
    create::create,
    get::get,
    sse::{get_sse, post_start_roulette, EventSender},
};
use data::Roulette;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let roulette_mutex: Data<Mutex<HashMap<String, Roulette>>> =
        Data::new(Mutex::new(HashMap::new()));
    let sse_mutex: Data<Arc<Mutex<EventSender>>> = Data::new(EventSender::new());

    HttpServer::new(move || {
        App::new()
            .service(create)
            .service(get)
            .service(get_sse)
            .service(post_start_roulette)
            .app_data(roulette_mutex.clone())
            .app_data(sse_mutex.clone())
    })
    .bind(("0.0.0.0", 8180))?
    .run()
    .await
}
