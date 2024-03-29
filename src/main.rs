mod api;
mod data;
mod event_sender;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use actix_cors::Cors;
use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use api::{create::create, get::get, sse::get_sse, start::post_start_roulette};
use data::Roulette;
use event_sender::EventSender;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let roulette_mutex: Data<Mutex<HashMap<String, Roulette>>> =
        Data::new(Mutex::new(HashMap::new()));
    let sse_mutex: Data<Arc<Mutex<EventSender>>> = Data::new(EventSender::new());

    HttpServer::new(move || {
        let cors = if cfg!(debug_assertions) {
            Cors::default().allow_any_origin()
        } else {
            Cors::default()
        };

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
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
