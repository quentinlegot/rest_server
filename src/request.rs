use crate::Method;
use std::collections::HashMap;

/// Request struct, used to represent a HTTP request send to the server.
#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl Request {

    /// Create a new Request struct
    /// Headers are parsed in parse_headers() method before return the Request object.
    pub fn new(method: Method, path: String, body: String) -> Self {
        let headers = Self::parse_header(body.as_str());
        Self {
            method,
            path,
            headers,
            body,
        }
    }

    /// Give the header value from the request body, key is the header name.
    /// Return a Option object containing the header value, if the header is not found, return None.
    pub fn get_header(&self, key: &str) -> Option<&str> {
        self.headers.get(key).map(|s| s.as_str())
    }

    /// Parse the headers of the request and return a HashMap containing the headers, key is the header name and value is the header value.
    fn parse_header(body: &str) -> HashMap<String, String> {
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