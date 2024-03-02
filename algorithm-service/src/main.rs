mod service;
mod gemma;
mod file_finder;
mod chat_gpt;

// TODO. Folder 를 순회하면서 env file 을 찾고, config Map 에 전달하는 것을 목적
fn main() {
    service::run();
}
