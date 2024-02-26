pub mod gemma_service {
    use std::env;
    use std::io;
    use std::io::Read;
    use std::fs::File;

    use crate::gemma::response::*;
    use crate::gemma::{self, *};
    use std::collections::HashMap;

    struct Args {
        pub configs: HashMap<String,String>,
    }

    impl Args {
        fn new(items: &Vec<&str>) -> Args {
            let mut map:HashMap<String,String> = HashMap::new();
            for item in items {
                let split_item: Vec<&str> = item.split("=").collect();
                map.insert(
                    split_item.get(0).expect("not Found").to_string(),
                    split_item.get(1).expect("msg").to_string(),
                );    
            }
            Args {
                configs: map
            }
        }
    }

    pub fn run() {
        let env: Vec<String> = env::args().collect();
        let mut file: File = File::open(&env[1]).expect(&format!("file-open-fail : {}", &env[1]));
        let mut env_file_result = String::new();

        file.read_to_string(&mut env_file_result)
            .expect("read file error");
        let env_vector: Vec<&str> = env_file_result.split("\r\n").collect();
        run_with_args(&Args::new(&env_vector));
    }

    fn run_with_args(args: &Args) -> ! {
        let api_key: String = args
            .configs
            .get("GOOGLE_API")
            .expect("NOT FOUND")
            .to_string();

        let gemma_api = GemmaAPI::new(api_key);
        loop {
            let cli_read = read_string();
            let gemma_result = gemma_api.talk_to_gemma_with_text(cli_read);

            let candidates = match gemma_result {
                Some(result) => result.candidates,
                None => vec![],
            };

            let contents = match candidates.get(0) {
                Some(candidate) => Option::Some(&candidate.content),
                None => Option::None,
            };

            let parts: Vec<Part> = match contents {
                Some(content) => content.parts.clone(),
                None => vec![],
            };

            match parts.get(0) {
                Some(text) => match text {
                    gemma::response::Part::text(value) => {
                        println!("response \n {}", value);
                    }
                    _ => println!("NONE Response"),
                },
                None => println!("NONE Response"),
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
}
