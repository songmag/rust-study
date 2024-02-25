pub mod response;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::http_util;

pub use self::response::*;

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
    parts: Vec<Part>,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct GemmaTextRequest {
    contents: Vec<Content>,
}

pub struct GemmaAPI {
    client: http_util::HttpClient,
    api_key: String
}

impl GemmaAPI {
    pub fn new(api_key: String) -> GemmaAPI {
        GemmaAPI {
            client: http_util::HttpClient::new(http_util::RequestHost::new(
                "https".to_string(),
                "generativelanguage.googleapis.com".to_string(),
                443,
            )),
            api_key
        }
    }

    pub fn talk_to_gemma_with_text(&self, text: String) -> Option<GemmaResponse> {
        if text.trim().is_empty() {
            return None;
        }
        return Some(
            self
            .client
            .post(
                &[],
                &format!("/v1beta/models/gemini-pro:generateContent?key={}", self.api_key),
                None,
                GemmaTextRequest {
                    contents: vec![Content {
                        parts: vec![Part::TextPart {
                            string: text.clone(),
                        }],
                    }],
                },
            )
            .expect("can not read")
        )
    }
}

#[cfg(test)]
mod test_http {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_gemma_serde() {
        let gemma_text_request = GemmaTextRequest {
            contents: vec![Content {
                parts: vec![Part::TextPart {
                    string: "test_string".to_string(),
                }],
            }],
        };

        let str_result = serde_json::to_string(&gemma_text_request).expect("error serde");

        print!("{:?}", str_result);
    }
}
