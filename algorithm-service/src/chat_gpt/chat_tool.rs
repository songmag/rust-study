use crate::chat_gpt::{ChatGPTService, ChatGPT};
use super::request::Message;

pub struct ChatToolChatGPT {
    chat_gpt: ChatGPT,
    prompt: Vec<Message>
}

impl ChatToolChatGPT {
    pub fn new(chat_gpt: ChatGPT) -> ChatToolChatGPT {
        return ChatToolChatGPT {
            chat_gpt,
            prompt : vec![]
        }
    }

    fn generate_message(&mut self, text: String) -> Vec<Message> {
        self.prompt.push(Message::new_user_message(text.to_string()));
        return self.prompt.to_vec();
    }
}

impl ChatGPTService for ChatToolChatGPT {
    
    fn add_prompt(&mut self,text: String) {
        self.prompt.push(Message::new_response(text.to_string()));
    }

    fn send(&mut self,text: &String) -> String {
        if text.trim().len() == 0 {
            return String::from("No Response");
        }
        let message = self.generate_message(text.to_string());
        // print!("{:?}",message);
        let completion = self.chat_gpt.send_message_gpt4(message);
        let item = if let Some(completion) = completion {
            self.add_prompt(completion.to_string());
            completion.to_string()
        } else {
            print!("Error Response...");
            String::new()
        };

        item
    }
}

