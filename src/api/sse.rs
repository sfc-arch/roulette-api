use std::sync::{Arc, Mutex};

use actix_web::{
    get,
    web::{self, Data},
    Responder,
};

use crate::event_sender::EventSender;

#[get("/api/roulette/{id}/sse")]
pub async fn get_sse(
    event_sender: Data<Arc<Mutex<EventSender>>>,
    id: web::Path<String>,
) -> impl Responder {
    event_sender
        .lock()
        .unwrap()
        .new_client(id.to_string())
        .await
}
