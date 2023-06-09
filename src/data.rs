use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RouletteItem {
    pub label: String,
}

#[derive(Serialize, Deserialize)]
pub struct Roulette {
    pub id: String,
    pub title: String,
    pub items: Vec<RouletteItem>,
    pub created_at: String,
    pub secret: String,
    pub result: Option<usize>,
}

#[derive(Serialize, Deserialize)]
pub struct RouletteCreateRequest {
    pub title: String,
    pub items: Vec<RouletteItem>,
}

#[derive(Serialize, Deserialize)]
pub struct RouletteCreateResponse {
    pub id: String,
    pub title: String,
    pub items: Vec<RouletteItem>,
    pub created_at: String, // UNIX Time
    pub secret: String,
}

#[derive(Serialize, Deserialize)]
pub struct RouletteGetResponse {
    pub id: String,
    pub title: String,
    pub items: Vec<RouletteItem>,
    pub created_at: String,
    pub result: Option<usize>,
}
