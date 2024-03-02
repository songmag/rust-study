pub mod response;
mod test_tool;
mod chat_tool;
mod request;

use http_util;

pub use self::response::*;
pub use self::request::*;

pub use self::chat_tool::*;

pub trait GeminiAPIService {
    fn add_prompt(&mut self,text: String);
    fn send(&self,text: &String) -> Option<GemmaResponse>;
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

    pub fn talk_to_gemma_with_text(&self, text: &String) -> Option<GemmaResponse> {
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
                    contents: vec![request::Content {
                        parts: vec![request::Part::TextPart {
                            string: text.clone(),
                        }],
                    }],
                },
            )?
        )
    }
}

#[cfg(test)]
mod test_http {
    use super::*;

    #[test]
    fn test_gemma_serde() {
        let gemma_text_request = GemmaTextRequest {
            contents: vec![request::Content {
                parts: vec![request::Part::TextPart {
                    string: "test_string".to_string(),
                }],
            }],
        };

        let str_result = serde_json::to_string(&gemma_text_request).expect("error serde");

        print!("{:?}", str_result);
    }
}