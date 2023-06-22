use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use actix_web::{
    http::StatusCode,
    post,
    web::{self, Data},
    HttpResponseBuilder, Responder,
};

use crate::{
    data::{Roulette, RouletteStartRequest},
    event_sender::EventSender,
};

#[post("/api/roulette/{id}/start")]
pub async fn post_start_roulette(
    event_sender: Data<Arc<Mutex<EventSender>>>,
    roulette_mutex: Data<Mutex<HashMap<String, Roulette>>>,
    id: web::Path<String>,
    req: web::Json<RouletteStartRequest>,
) -> impl Responder {
    if let Some(roulette) = roulette_mutex.lock().unwrap().get_mut(&id.to_string()) {
        if roulette.secret == req.0.secret {
            let r = roulette.run();

            event_sender
                .lock()
                .unwrap()
                .send(&format!("start {r}"), id.to_string())
                .await;

            roulette.result = Some(r);

            HttpResponseBuilder::new(StatusCode::OK)
                .content_type("text/plain")
                .body(format!("{r}"))
        } else {
            HttpResponseBuilder::new(StatusCode::UNAUTHORIZED)
                .content_type("text/plain")
                .body("Unauthorized")
        }
    } else {
        HttpResponseBuilder::new(StatusCode::NOT_FOUND)
            .content_type("text/plain")
            .body("Not Found")
    }
}
