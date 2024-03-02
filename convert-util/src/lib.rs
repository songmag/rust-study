use html2md::parse_html;
use std::io::{Write,Read};

pub fn html2md(html:String) -> String {
    parse_html(&html)
}

#[cfg(test)]
mod tests {
    use std::{vec, fs::File};

    use super::*;

    #[test]
    fn it_works() {
        let client = http_util::HttpClient::new(
            http_util::RequestHost::new("https".to_string()
                ,"www.javadoc.io".to_string(),
                443)
        );
        //let result: String = client.get_to_string(&vec![], "/doc/org.assertj/assertj-core/latest/org/assertj/core/api/Assertions.html".unwrap())


        let mut file:File = std::fs::File::open("./Assertions.html").unwrap();

        // 파일 크기 추출
        let metadata = file.metadata().unwrap();
        let file_size = metadata.len() as usize;

        // 파일 크기만큼의 버퍼 할당
        let mut buffer = vec![0; file_size];

        // 파일을 버퍼에 읽어들임
        file.read_exact(&mut buffer);
        let content:String = String::from_utf8_lossy(&buffer).to_string();

        println!("{}", content);
        let mut write_file:File = std::fs::File::open("./assertj.md").unwrap_or_else(
            |_| File::create("./assertj.md").expect("error create file")
        );
        let md_str = html2md(content);
        write_file.write_all(&md_str.into_bytes());
    }
}
