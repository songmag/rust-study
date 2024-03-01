
use serde::{Deserialize, Serialize};

#[derive( Debug, Deserialize,Clone)]
pub enum Part {
    text(String)
}
#[derive( Debug, Deserialize)]
pub struct Content {
    pub parts: Vec<Part>,
    pub role: String
}
#[derive( Debug, Deserialize)]
pub struct Candidate {
    pub content: Content
}
#[derive( Debug, Deserialize)]
pub struct GemmaResponse {
    pub candidates: Vec<Candidate>
}