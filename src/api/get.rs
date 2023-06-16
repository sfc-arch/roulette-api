use std::{collections::HashMap, sync::Mutex};

use actix_web::{
    get,
    http::StatusCode,
    web::{self, Data},
    HttpResponse,
};

use crate::data::Roulette;

#[get("/api/roulette/{id}")]
pub async fn get(
    roulette_mutex: Data<Mutex<HashMap<String, Roulette>>>,
    id: web::Path<String>,
) -> HttpResponse {
    let roulette_mutex = roulette_mutex.lock().unwrap();
    let roulette = roulette_mutex.get(&id.to_string());

    if let Some(roulette) = roulette {
        HttpResponse::Ok()
            .content_type("application/json")
            .body(serde_json::to_string(&roulette.to_get_response()).unwrap())
    } else {
        HttpResponse::build(StatusCode::NOT_FOUND)
            .content_type("text/plain")
            .body("Not Found")
    }
}
