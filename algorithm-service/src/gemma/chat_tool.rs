use crate::gemma::GemmaAPI;
use crate::gemma::GemmaResponse;
use std::io::Read;

use super::GeminiAPIService;


pub struct ChatToolGemini {
    gemma_api : GemmaAPI,
    prompt: String
}

impl ChatToolGemini {
    pub fn new(api : GemmaAPI) -> ChatToolGemini {
        api.talk_to_gemma_with_text("너의 모든 대화 응답은 [ 와 ] 로 감싸져 있어 내가 하는 말은 [] 로 안 감싸져 있는 글자야, 그리고 이전 대화 내용은 ___로 시작하고 ___로 끝나".to_string());
        ChatToolGemini {
            gemma_api : api,
            prompt: String::from("너의 모든 대화 응답은 [ 와 ] 로 감싸져 있어 내가 하는 말은 [] 로 안 감싸져 있는 글자야, 그리고 이전 대화 내용은 ___로 시작하고 ___로 끝나")
        }
    }
}

impl GeminiAPIService for ChatToolGemini {
    fn add_prompt(&mut self, text: String) {
        self.prompt = format!("{}\n{}", self.prompt, text);
    }

    fn send(&self, text: &String) -> Option<GemmaResponse> {
        self.gemma_api.talk_to_gemma_with_text(format!("___{}___\n{}", self.prompt, text))
    }
}

