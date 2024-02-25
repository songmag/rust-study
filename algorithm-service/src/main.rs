use algorithm;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io;
use std::io::Read;

use crate::gemma::GemmaAPI;
mod gemma;
mod http_util;

// TODO. Folder 를 순회하면서 env file 을 찾고, config Map 에 전달하는 것을 목적
fn main() {
    let env: Vec<String> = env::args().collect();
    let mut file: File = File::open(&env[1]).expect(&format!("file-open-fail : {}", &env[1]));
    let mut env_file_result = String::new();
    let mut config_map: HashMap<String, String> = HashMap::new();

    file.read_to_string(&mut env_file_result)
        .expect("read file error");
    for item in env_file_result.split("\r\n") {
        let split_item: Vec<&str> = item.split("=").collect();
        config_map.insert(
            split_item.get(0).expect("not Found").to_string(),
            split_item.get(1).expect("msg").to_string(),
        );
    }

    let api_key: String = config_map.get("GOOGLE_API").expect("NOT FOUND").to_string();

    let gemmaAPI = GemmaAPI::new(api_key);

    'cli_command: loop {
        let cli_read = read_string();
        if let gemma::response::Part::text(value) = gemmaAPI.talk_to_gemma_with_text(cli_read).expect("")
            .candidates
            .get(0)
            .expect("not_exists")
            .content
            .parts
            .get(0)
            .expect("not text") {
                println!("response :: {:?}", value);
            } else {
                println!("NONE Response")
            }
        };
}

pub fn read_string() -> String {
    let mut string = String::new();
    let _ = io::stdin()
        .read_line(&mut string)
        .expect("Can Not Read String in Standard I/O");

    return string;
}
