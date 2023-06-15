use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RouletteItem {
    pub label: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Roulette {
    pub id: String,
    pub title: String,
    pub items: Vec<RouletteItem>,
    pub created_at: String,
    pub secret: String,
    pub result: Option<usize>,
}

impl Roulette {
    pub fn from_response(response: RouletteCreateResponse) -> Self {
        Self {
            id: response.id,
            title: response.title,
            items: response.items,
            created_at: response.created_at,
            secret: response.secret,
            result: None,
        }
    }

    pub fn to_get_response(&self) -> RouletteGetResponse {
        RouletteGetResponse {
            id: self.id.clone(),
            title: self.title.clone(),
            items: self.items.clone(),
            created_at: self.created_at.clone(),
            result: self.result,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RouletteCreateRequest {
    pub title: String,
    pub items: Vec<RouletteItem>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RouletteCreateResponse {
    pub id: String,
    pub title: String,
    pub items: Vec<RouletteItem>,
    pub created_at: String, // UNIX Time
    pub secret: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RouletteGetResponse {
    pub id: String,
    pub title: String,
    pub items: Vec<RouletteItem>,
    pub created_at: String,
    pub result: Option<usize>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RouletteStartRequest {
    pub secret: String,
}
