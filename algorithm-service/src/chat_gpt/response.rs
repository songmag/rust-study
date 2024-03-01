use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Message {
    // role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
pub struct Choice {
    message: Message,
    // logprobs: Option<serde_json::Value>,
    // finish_reason: String,
    // index: usize,
}

#[derive(Debug, Deserialize)]
pub struct Usage {
    // prompt_tokens: usize,
    // completion_tokens: usize,
    // total_tokens: usize,
}


#[derive(Debug, Deserialize)]
pub struct ChatCompletion {
    // id: String,
    // object: String,
    // created: i64,
    // model: String,
    // usage: Usage,
    choices: Vec<Choice>,
}

impl ChatCompletion {
    pub fn to_string(&self) -> String {
        let content_vec: Vec<String> = self
            .choices
            .iter()
            .map(|choice| choice.message.content.clone())
            .collect();
    
        content_vec.join("\n")
    }
}


#[cfg(test)]
mod test_deseirialize {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_chat_response_serde() {
        let json_str = r#"
        {
            "id": "chatcmpl-abc123",
            "object": "chat.completion",
            "created": 1677858242,
            "model": "gpt-3.5-turbo-0613",
            "usage": {
                "prompt_tokens": 13,
                "completion_tokens": 7,
                "total_tokens": 20
            },
            "choices": [
                {
                    "message": {
                        "role": "assistant",
                        "content": "\n\nThis is a test!"
                    },
                    "logprobs": null,
                    "finish_reason": "stop",
                    "index": 0
                }
            ]
        }
    "#;

    let chat_completion: ChatCompletion = serde_json::from_str(json_str).unwrap();
        print!("{:?}", chat_completion);
    }
}


