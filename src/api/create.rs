use chrono::Utc;
use std::{collections::HashMap, sync::Mutex, time::Instant};
use uuid::Uuid;

use actix_web::{
    post,
    web::{self, Data},
};

use crate::data::{Roulette, RouletteCreateRequest, RouletteCreateResponse};

#[post("/api/roulette")]
pub async fn create(
    roulette_mutex: Data<Mutex<HashMap<String, Roulette>>>,
    req: web::Json<RouletteCreateRequest>,
) -> String {
    let roulette_id = Uuid::new_v4().to_string();
    let secret = Uuid::new_v4().to_string();
    let created_at = Utc::now();

    let roulette = RouletteCreateResponse {
        id: roulette_id.clone(),
        secret,
        title: req.title.clone(),
        items: req.items.clone(),
        created_at: created_at.to_string(),
    };

    let mut roulette_mutex = roulette_mutex.lock().unwrap();
    roulette_mutex.insert(roulette_id, Roulette::from_response(roulette.clone()));

    serde_json::to_string(&roulette).unwrap()
}
