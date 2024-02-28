use crate::gemma::GemmaAPI;
use crate::gemma::GemmaResponse;
use std::io::Read;

use super::GeminiAPIService;


pub struct ChatToolGemini {
    gemma_api : GemmaAPI,
    prompt: String,
}

impl ChatToolGemini {
    pub fn new(api : GemmaAPI) -> ChatToolGemini {
        ChatToolGemini {
            gemma_api : api,
            prompt: String::from("")
        }
    }
}

impl GeminiAPIService for ChatToolGemini {
    fn add_prompt(&mut self, text: String) {
        self.prompt = format!("{}\n{}", self.prompt, text);
    }

    fn send(&self, text: &String) -> Option<GemmaResponse> {
        self.gemma_api.talk_to_gemma_with_text(format!("{}\n{}", self.prompt, text))
    }
}

