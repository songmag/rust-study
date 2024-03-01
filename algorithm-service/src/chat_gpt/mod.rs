mod response;
mod request;
pub mod chat_tool;
use crate::http_util::{HttpClient, RequestHost};
pub use chat_tool::ChatToolChatGPT;
pub trait ChatGPTService {
    fn add_prompt(&mut self,text: String);
    fn send(&mut self,text: &String) -> String;
}

pub struct ChatGPT {
    client: HttpClient,
    api_key: String
}

impl ChatGPT {
    pub fn new(api_key: String) -> ChatGPT {
         let client = HttpClient::new(RequestHost::new(
            "https".to_string(),
            "api.openai.com".to_string(),
            443,
        ));
        ChatGPT {
            client, api_key
        }
    }

    pub fn send_message(&self,messages:Vec<request::Message>) -> Option<response::ChatCompletion>{
        self.client.post(
            &vec![("Authorization".to_string(), format!("Bearer {}", self.api_key))], 
            "/v1/chat/completions",
            None,
            request::Request::new_chat_gpt(messages)
        )
    }
}