use crate::gemma::GemmaAPI;
use crate::gemma::GemmaResponse;

use super::GeminiAPIService;


pub struct ChatToolGemini {
    gemma_api : GemmaAPI,
    prompt: String
}

impl ChatToolGemini {
    pub fn new(api : GemmaAPI) -> ChatToolGemini {
        let init_prompt :String  = "All of your responses are enclosed in [ and ], while my responses are not enclosed in brackets. The text you generate is not enclosed in brackets either. The previous conversation starts with ___ and ends with ___. Please enclose your response in [ and ].".to_string();
        api.talk_to_gemma_with_text(&init_prompt);
        ChatToolGemini {
            gemma_api : api,
            prompt: init_prompt
        }
    }
}

impl GeminiAPIService for ChatToolGemini {
    fn add_prompt(&mut self, text: String) {
        self.prompt = format!("{}\n{}", self.prompt, text);
    }

    fn send(&self, text: &String) -> Option<GemmaResponse> {
        println!("Request... ... ");
        if text.trim().len() == 0 {
            return None
        }
        self.gemma_api.talk_to_gemma_with_text(&format!("___{}___\n{}", self.prompt, text))
    }
}

