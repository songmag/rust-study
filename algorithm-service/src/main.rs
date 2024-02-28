mod service;
mod gemma;
mod http_util;
mod file_finder;

// TODO. Folder 를 순회하면서 env file 을 찾고, config Map 에 전달하는 것을 목적
fn main() {
    service::gemma_service::run();
}
