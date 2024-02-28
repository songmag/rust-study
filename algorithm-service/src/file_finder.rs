use std::fs;
pub fn find_files(dir_path: &str, target_file: &str) -> Vec<fs::File> {
    let mut result_files = Vec::new();

    if let Ok(entries) = fs::read_dir(dir_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();

                if path.is_file() {
                    if let Some(file_name) = path.file_name() {
                        if file_name == target_file {
                            if let Ok(file) = fs::File::open(&path) {
                                result_files.push(file);
                            }
                        }
                    }
                } else if path.is_dir() {
                    // 재귀적으로 디렉토리를 순회
                    let sub_dir_path = path.to_str().unwrap_or_default();
                    let sub_dir_files = find_files(sub_dir_path, target_file);
                    result_files.extend(sub_dir_files);
                }
            }
        }
    }

    result_files
}