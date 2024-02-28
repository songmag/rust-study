
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize,Clone)]
pub enum Part {
    text(String)
}
#[derive(Serialize, Debug, Deserialize)]
pub struct Content {
    pub parts: Vec<Part>,
    pub role: String
}
#[derive(Serialize, Debug, Deserialize)]
pub struct Candidate {
    pub content: Content
}
#[derive(Serialize, Debug, Deserialize)]
pub struct GemmaResponse {
    pub candidates: Vec<Candidate>
}