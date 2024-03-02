use reqwest::{blocking, header::HeaderMap};
use serde::{Deserialize, Serialize};
use std::{error::Error};

pub struct HttpClient {
    request_host: RequestHost,
}

pub struct RequestHost {
    protocol: String,
    host: String,
    port: i128,
}

impl RequestHost {
    pub fn new(
        protocol : String,
        host: String,
        port: i128
    ) -> RequestHost {
        RequestHost { protocol, host, port }
    }
    pub fn get_base_url(&self) -> String {
        format!("{}://{}:{}", &self.protocol, &self.host, &self.port)
    }
}


pub struct RequestHttp {
    uri: String,
    headers: HeaderMap,
    body: Option<String>,
}

fn make_headers(headers :&[(String, String)]) -> HeaderMap {
    let mut maps:HeaderMap = HeaderMap::new();

    for header in headers {
        if header.0 == "Authorization" {
            maps.append(reqwest::header::AUTHORIZATION, header.1.parse().unwrap());
        }
    }
    return maps;
}

impl HttpClient {
    pub fn new(request_host:RequestHost) -> HttpClient {
        HttpClient { request_host }
    }

    pub fn get<T>(&self, headers: &[(String, String)],path: &str, path_param: Option<Vec<(String, String)>>) -> Option<T>
    where
        T: Serialize + for<'a> Deserialize<'a>,
    {
        let str_path = match path_param {
            Some(params) => generate_path(path, &params),
            None => path.to_string(),
        };

        let headers = make_headers(headers);

        let result = RequestHttp::new(
            &self.request_host, 
            Some(&str_path),
            headers,
            None
        );

        if let Ok(item) = result.get_with_http() {
            return Some(serde_json::from_str(&item).expect(&format!("unwrap error { } ", &item)));
        }
        return None;
    }

    pub fn get_to_string(&self, headers: &[(String, String)],path: &str, path_param: Option<Vec<(String, String)>>) -> Option<String> {
        let str_path = match path_param {
            Some(params) => generate_path(path, &params),
            None => path.to_string(),
        };

        let headers = make_headers(headers);

        let result = RequestHttp::new(
            &self.request_host, 
            Some(&str_path),
            headers,
            None
        );

        if let Ok(item) = result.get_with_http() {
            return Some(item);
        }
        return None;    
    }


    pub fn post<T, R>(&self, headers:&[(String, String)], path: &str, path_param: Option<Vec<(String, String)>>,body: T) -> Option<R>
    where
        T: Serialize,
        R: for<'a> Deserialize<'a>,
    {

        let str_path = match path_param {
            Some(params) => generate_path(path, &params),
            None => path.to_string(),
        };

        let result = RequestHttp::new(
            &self.request_host, 
            Some(&str_path),
            make_headers(headers),
            Some(&serde_json::to_string(&body).ok()?)
        );
        let str_result = result.post_with_http().unwrap();
        
        Some(
            serde_json::from_str::<R>(&str_result).expect(&format!("Error Parsing \n :: {}", str_result))
        )
    }
}

impl RequestHttp {
    pub fn new(
        host: &RequestHost,
        path: Option<&String>,
        headers: HeaderMap,
        body: Option<&String>,
    ) -> RequestHttp {
        return RequestHttp {
            uri: match path {
                Some(item) => {
                    format!("{}/{}", host.get_base_url(), item)
                }
                None => host.get_base_url(),
            },
            headers: headers,
            body: match body {
                Some(item) => Some(item.clone()),
                None => None,
            },
        };
    }

    pub fn post_with_http(&self) -> Result<String, Box<dyn Error>> {
        let client_builder = blocking::ClientBuilder::new();
        let client = client_builder.timeout(std::time::Duration::from_secs(60)).build().unwrap();
        let rsp = client.post(&self.uri)
        .header("content-type", "application/json")
        .headers(self.headers.to_owned())
        .body(match &self.body {
            Some(item) => {
                item.clone()
            }
            None => "".to_string()
               })
        .send()
        .expect(&format!("Fail To Send Message {}", &self.uri));

        Ok(rsp.text().expect("parsing exception response to text"))
    }


    pub fn get_with_http(&self) -> Result<String, Box<dyn Error>> {
        let client_builder = blocking::ClientBuilder::new();
        let client = client_builder.timeout(std::time::Duration::from_secs(60)).build().unwrap();
        println!("{}", &self.uri);
        let response = client.get(&self.uri)
            .headers(self.headers.to_owned())
            .send()?;
        let body = response.text().unwrap(); 
        Ok(body)
    }
}

pub fn generate_path(path: &str, path_param: &Vec<(String, String)>) -> String {
    let mut replace_path: String = path.to_string();
    for (key, value) in path_param {
        replace_path = replace_path.replace(&("{".to_string() + key + &"}"), value);
    }

    replace_path
}

#[cfg(test)]
mod test_http {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    struct JsonPlaceHolderData {    
        userId: i32,
        id: i32,
        title: String,
        completed: bool,
    }

    #[test]
    fn test_request() {
        let client = HttpClient {
            request_host: RequestHost {
                protocol: "https".to_string(),
                host: "jsonplaceholder.typicode.com".to_string(),
                port: 443,
            },
        };

        let result: Option<JsonPlaceHolderData> = client.get(&[],"/todos/1", None);
        println!("{:?}", result);
    }

    #[test]
    fn test_generate_path() {
        println!(
            "{}",
            generate_path(
                "/users/{user_id}/klassId/{klassId}",
                &vec![
                    ("user_id".to_string(), "t1".to_string()),
                    ("klassId".to_string(), "t2".to_string())
                ]
            )
        );
    }
}
