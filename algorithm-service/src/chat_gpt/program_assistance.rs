use crate::chat_gpt::request::Message;
use std::fs::File;
use std::path::PathBuf;
use std::io::Read;

pub fn read_file(file:&mut Box<File>, chunk_size: usize) -> Vec<String>{
    let mut buff = String::new();
    let _ = file.read_to_string(&mut buff);

    buff.chars()
    .collect::<Vec<_>>()
    .chunks(chunk_size)
    .map(|chunk| chunk.iter().collect::<String>())
    .collect()
}

pub fn file_filter(file_info:Box<&(Box<PathBuf>, Box<File>)>) -> bool {
    let file_name = String::from(file_info.0.as_path().file_name().unwrap().to_str().unwrap());
    let filtered_file_name = vec!["Repository", "ErrorCode", "Test"];
    
    filtered_file_name.iter()
        .any(| &x| file_name.contains(x))
}
