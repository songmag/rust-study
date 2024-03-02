use std::fs::File;
use std::fs;
use std::path::{Path, PathBuf};
use regex::Regex;

pub fn find_files(dir_path: &str, target_file: &str) -> Vec<(Box<PathBuf>, Box<File>)> {
    let mut result_files = Vec::new();
    let target_file_regex = Regex::new(format!(r"{}",target_file).as_str()).unwrap();

    if let Ok(entries) = fs::read_dir(dir_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();

                if path.is_file() {
                    if let Some(file_name) = path.file_name() {
                        if let Some(file_name_str) = file_name.to_str() {
                            if target_file_regex.is_match(file_name_str) {
                                if let Ok(file) = File::open(&path) {
                                    result_files.push((Box::new(Path::new(&path).to_path_buf()), Box::new(file)));
                                }
                            }
                        }
                    }
                } else if path.is_dir() {
                    // 재귀적으로 디렉토리를 순회
                    let sub_dir_path: &str = path.to_str().unwrap_or_default();
                    let sub_dir_files = find_files(sub_dir_path, target_file);
                    result_files.extend(sub_dir_files);
                }
            }
        }
    }

    result_files
}


#[cfg(test)]
mod test_find_file {
    use super::*;
    use std::{fs::File, io::Write};

    #[test]
    fn find_file() {
        let target_file_regex = Regex::new(format!(r"{}","[.]*.java").as_str()).unwrap();
        let p = "CreateReceipt.java";
        print!("{}",target_file_regex.is_match(p));
    }
}