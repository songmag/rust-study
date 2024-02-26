use reqwest::blocking;
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
        return format!("{}://{}:{}", &self.protocol, &self.host, &self.port);
    }
}

pub enum HttpMethod {
    GET,
    POST,
    PATCH,
    PUT,
}

pub struct RequestHttp {
    uri: String,
    method: HttpMethod,
    body: Option<String>,
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

        let result = RequestHttp::new(HttpMethod::GET,
            &self.request_host, 
            Some(&str_path),
            None
        );

        if let Ok(item) = result.get_with_http() {
            return Some(serde_json::from_str(&item).unwrap());
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

        let result = RequestHttp::new(HttpMethod::POST,
            &self.request_host, 
            Some(&str_path),
            Some(&serde_json::to_string(&body).ok()?)
        );

        let str_result = result.post_with_http().unwrap();
        
        Some(
            serde_json::from_str::<R>(&str_result).expect("Post Result To R")
        )
    }
}

impl RequestHttp {
    pub fn new(
        method: HttpMethod,
        host: &RequestHost,
        path: Option<&String>,
        body: Option<&String>,
    ) -> RequestHttp {
        return RequestHttp {
            uri: match path {
                Some(item) => {
                    format!("{}/{}", host.get_base_url(), item)
                }
                None => host.get_base_url(),
            },
            method,
            body: match body {
                Some(item) => Some(item.clone()),
                None => None,
            },
        };
    }

    pub fn post_with_http(&self) -> Result<String, Box<dyn Error>> {
        let client = blocking::Client::new();
        let rsp = client.post(&self.uri)
        .header("content-type", "application/json")
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
        Ok(blocking::get(&self.uri)?.text().unwrap())
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
                "/users/{userId}/klassId/{klassId}",
                &vec![
                    ("userId".to_string(), "t1".to_string()),
                    ("klassId".to_string(), "t2".to_string())
                ]
            )
        );
    }
}
