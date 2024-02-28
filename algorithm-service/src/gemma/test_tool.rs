use crate::gemma::GemmaAPI;
use crate::gemma::GemmaResponse;
use std::fs;
use std::io::Read;

use super::GeminiAPIService;

pub struct TestToolGemini {
    gemma_api : GemmaAPI,
    prompt: String
}

impl TestToolGemini {
    pub fn new(api : GemmaAPI) -> TestToolGemini {
        let files = crate::file_finder::find_files("./test", "pre-prompt.txt");
        let mut prompt = String::new();
        for mut file in files {
            let mut file_str = String::new();
            file.read_to_string(&mut file_str);
            prompt = format!("{}\n{}",prompt,file_str);
        }
        TestToolGemini {
            gemma_api : api,
            prompt
        }
    }
}

impl GeminiAPIService for TestToolGemini {
    fn add_prompt(&mut self,text: String) {
        todo!("프롬프트에 추가하는 로직을 넣는다")
    }

    fn send(&self,text: &String) -> Option<GemmaResponse> {
        self.gemma_api.talk_to_gemma_with_text(format!("{}\n{}", self.prompt, text))
    }
}

