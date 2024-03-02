use super::request::Message;
use crate::chat_gpt::program_assistance;
use crate::{
    chat_gpt::{ChatGPT, ChatGPTService},
    file_finder::find_files,
};
use std::{
    fs::File,
    io::{Read, Write},
};

pub struct ProgrammingToolChatGPT {
    chat_gpt: ChatGPT,
    prompt: Vec<Message>,
    workspace: String,
}

impl ProgrammingToolChatGPT {
    pub fn new(
        chat_gpt: ChatGPT,
        files: &mut Vec<Box<File>>,
        workspace: String,
    ) -> ProgrammingToolChatGPT {
        let mut prompt = Vec::new();
        for file in files {
            prompt.append(
                &mut program_assistance::read_file(file, 4096)
                    .iter()
                    .map(|it| Message::new_prompt(it.to_string()))
                    .collect(),
            );
        }

        println!("{:?}", prompt);

        return ProgrammingToolChatGPT {
            chat_gpt,
            prompt,
            workspace,
        };
    }

    pub fn add_prompt_from_file(&mut self, file: &mut Box<File>) {
        self.prompt.append(
            &mut program_assistance::read_file(file, 4096)
                .iter()
                .map(|it| Message::new_prompt(it.to_string()))
                .collect(),
        );
    }

    fn generate_message(&mut self, file: Box<File>) -> Vec<Message> {
        let mut result_message = vec![];

        result_message.append(&mut self.prompt.to_vec());
        result_message.push(Message::new_user_message("```java".to_string()));
        result_message.append(&mut self.make_files_message(vec![file]));
        result_message.push(Message::new_user_message("```".to_string()));
        result_message.push(Message::new_user_message(
            "Write the test case.".to_string(),
        ));

        return result_message.to_vec();
    }

    fn make_files_message(&mut self, files: Vec<Box<File>>) -> Vec<Message> {
        let mut result_message = vec![];
        for mut file in files {
            let messages = program_assistance::read_file(&mut file, 4096);

            let mut messages: Vec<Message> = messages
                .iter()
                .map(|st| Message::new_user_message(st.to_string()))
                .collect();
            result_message.append(&mut messages);
        }

        result_message
    }
}

impl ChatGPTService for ProgrammingToolChatGPT {
    fn add_prompt(&mut self, text: String) {}

    fn send(&mut self, text: &String) -> String {
        if text.trim().len() == 0 {
            return String::from("No Response");
        }

        let fixture_files: Vec<(Box<std::path::PathBuf>, Box<File>)> =
            find_files(&self.workspace, "[.]*Fixture.java");

        for mut file in fixture_files {
            self.add_prompt_from_file(&mut file.1);
        }

        let mut files = find_files(
            "/Users/mark-song/class101/new-commerce/merchant/core/src/main/java",
            text,
        );

        if files.is_empty() {
            return String::from("No File");
        }

        for mut file_info in files {
            if program_assistance::file_filter(Box::new(&file_info)) {
                println!(
                    "Skip.. file {}",
                    file_info.0.file_name().unwrap().to_str().unwrap()
                );
                continue;
            }
            let (path, file) = file_info;
            let file_name = path.file_name().unwrap().to_str().unwrap();
            let split_java_file: Vec<&str> = file_name.split(".").collect();

            let mut result_file =
                File::create("./".to_string() + split_java_file.get(0).expect("Error") + "Test" + ".java")
                .expect("Can`t Create File");
            let message = self.generate_message(file);

            println!("{}", path.as_path().file_name().unwrap().to_str().unwrap());

            let completion = self.chat_gpt.send_message(message);
            let item = if let Some(completion) = completion {
                println!("{}", path.as_path().file_name().unwrap().to_str().unwrap());
                println!("{}", completion.to_string());
                completion.to_string()
            } else {
                print!("Error Response...");
                String::new()
            };
            result_file
                .write_all(item.as_bytes())
                .expect("Write File Error");
            result_file.flush();
        }

        "OK".to_string()
    }
}

#[cfg(test)]
mod test_progamming_tool {
    use super::*;
    use std::{fs::File, io::Write};

    #[test]
    fn new_chat_gpt() {
        let file: File = File::open("./assertj.md").unwrap();

        let chat_gpt: ProgrammingToolChatGPT =
            ProgrammingToolChatGPT::new(ChatGPT::new("".to_string()), vec![&mut Box::new(file)]);

        println!("{:#?}", chat_gpt.prompt);
    }
}
