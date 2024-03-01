use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    role: String,
    content: String,
}

impl Message {
    pub fn new_user_message(text: String) -> Message {
        Message {
            role: String::from("user"),
            content : text
        }
    }

    pub fn new_request(text:String) -> Message {
        Message {
            role : String::from("system"),
            content: text
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    model: String,
    messages: Vec<Message>,
    temperature: f64,
}

impl Request {
    pub fn new_chat_gpt(message : Vec<Message>) -> Request {
        Request {
            model : String::from("gpt-3.5-turbo"),
            temperature : 0.7,
            messages: message,
        }
    }
}