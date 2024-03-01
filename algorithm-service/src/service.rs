use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io;
use std::io::Read;

use crate::gemma::{ChatToolGemini, GeminiAPIService, GemmaAPI};

pub(crate) struct Args {
    pub configs: HashMap<String, String>,
}

impl Args {
    fn new() -> Args {
        let map: HashMap<String, String> = HashMap::new();
        Args { configs: map }
    }

    fn add(&mut self, items: Vec<String>) {
        for item in items {
            let split_item: Vec<&str> = item.split("=").collect();
            self.configs.insert(
                split_item.get(0).expect("not Found").to_string(),
                split_item.get(1).expect("msg").to_string(),
            );
        }
    }
}

pub fn run() {
    let system_args: Vec<String> = env::args().collect();
    println!("{:?}", system_args);
    let files: Vec<File> = crate::file_finder::find_files(".", ".env.config");
    let mut args: Args = Args::new();
    for mut file in files {
        let mut env_file_result = String::new();
        file.read_to_string(&mut env_file_result)
            .expect("read file error");
        let env_vector: Vec<String> = env_file_result
            .trim()
            .split("\n")
            .map(move |s| s.to_string())
            .collect();
        print!("{:?}", env_vector);
        args.add(env_vector);
    }
    match system_args.get(1) {
        Some(ai_module) => {
            if(ai_module == "chatgpt") {
                chat_service::run_with_args(&args);
            }
            else {
                gemma_service::run_with_args(&args);    
            }
        }
        None => {
            gemma_service::run_with_args(&args);
        },
    }
}

pub mod chat_service {
    use crate::{service, chat_gpt::{self, ChatToolChatGPT, ChatGPTService}};
    pub(crate) fn run_with_args(args: &service::Args) {
        let api_key: String = args
            .configs
            .get("CHAT_GPT_API")
            .expect("NOT FOUND Chat gpt Api key")
            .to_string();
        let chat_gpt = chat_gpt::ChatGPT::new(api_key);

        let mut chat_tool: Box<dyn ChatGPTService> = Box::new(ChatToolChatGPT::new(chat_gpt));

        loop {
            print!("reading... >>>\n");
            let cli_read = service::read_string();
            let response = chat_tool.send(&cli_read.trim().to_string());
            println!("resposne \n {}",response);
        }
    }
}

pub mod gemma_service {
    use crate::gemma::{self, *};
    use crate::service;

    use super::get_gemini_api;

    pub(crate) fn run_with_args(args: &service::Args) {
        let api_key: String = args
            .configs
            .get("GOOGLE_API")
            .expect("NOT FOUND google api key")
            .to_string();

        let gemma_api = GemmaAPI::new(api_key);
        print!("type 을 입력해주세요\n 1. chat \n 2. test_generator \n 3. code_generator \n");
        let cli_read = service::read_string();
        let opt_api = get_gemini_api(&cli_read, gemma_api);

        match opt_api {
            Some(mut api) => loop {
                print!("reading... >>>\n");
                let cli_read = service::read_string();

                let gemma_result = api.send(&cli_read);

                let candidates = match gemma_result {
                    Some(result) => result.candidates,
                    None => vec![],
                };

                let contents = match candidates.get(0) {
                    Some(candidate) => Option::Some(&candidate.content),
                    None => Option::None,
                };

                let parts: Vec<response::Part> = match contents {
                    Some(content) => content.parts.clone(),
                    None => vec![],
                };

                match parts.get(0) {
                    Some(text) => match text {
                        gemma::response::Part::text(value) => {
                            println!("response\n\n{}", value);
                            api.add_prompt(format!(
                                "{}\n{}",
                                cli_read.to_string(),
                                value.to_string()
                            ));
                        }
                    },
                    None => println!("NONE Response"),
                }
            },
            None => {
                print!("ERROR 지원하지 않는 타입입니다");
                return;
            }
        }
    }
}

pub fn read_string() -> String {
    let mut string = String::new();
    let _ = io::stdin()
        .read_line(&mut string)
        .expect("Can Not Read String in Standard I/O");

    return string;
}

fn get_gemini_api(request: &str, gemma_api: GemmaAPI) -> Option<Box<dyn GeminiAPIService>> {
    print!("{}", request);
    if request.trim() == "chat" {
        return Some(Box::new(ChatToolGemini::new(gemma_api)));
    }
    None
}
