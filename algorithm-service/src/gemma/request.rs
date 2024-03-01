use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize)]
pub enum Part {
    TextPart { string: String },
}

#[derive(Serialize, Debug, Deserialize)]
struct TPart {
    text: String,
}

impl Serialize for Part {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Part::TextPart { string } => serializer.serialize_newtype_struct(
                "",
                &TPart {
                    text: string.clone(),
                },
            ),
        }
    }
}

#[derive(Serialize, Debug, Deserialize)]
pub struct Content {
    pub parts: Vec<Part>,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct GemmaTextRequest {
    pub contents: Vec<Content>,
}
