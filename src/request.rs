use crate::Method;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl Request {
    pub fn new(method: Method, path: String, body: String) -> Self {
        let headers = Self::parse_header(body.as_str());
        Self {
            method,
            path,
            headers,
            body,
        }
    }

    pub fn get_header(&self, key: &str) -> Option<&str> {
        self.headers.get(key).map(|s| s.as_str())
    }

    pub fn parse_header(body: &str) -> HashMap<String, String> {
        let mut headers = HashMap::new();
        for line in body.split("\r\n") {
            let parts = line.split(": ").collect::<Vec<&str>>();
            if parts.len() == 2 {
                headers.insert(String::from(parts[0]), String::from(parts[1]));
            }
        };
        headers
    }
}