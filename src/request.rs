use crate::Method;
use std::collections::HashMap;
use core::fmt::Display;
use std::hash::{Hash, Hasher};

/// Request struct, used to represent a HTTP request send to the server.
#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub path: RequestPath,
    pub headers: HashMap<String, String>,
    body: String,
}

impl Request {

    /// Create a new Request struct.
    /// Headers are parsed in parse_headers() (private method) method before return the Request object.
    pub fn new(method: Method, path: RequestPath, body: String) -> Self {
        let headers = Self::parse_header(body.as_str());
        let body = body.split_once("\r\n\r\n").unwrap_or(("", "")).1.to_string().replace("\0", "");
        Self {
            method,
            path,
            headers,
            body,
        }
    }

    /// Give the header value from the request body, key is the header name.
    /// 
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

    /// Return the body of the request, the body hasn't been decoded (see decode_body() method).
    pub fn get_body(&self) -> String {
        self.body.clone()
    }

    /// Return the request body as a String map.
    /// 
    /// The key is the name of the form field, and the value is the value of the form field.
    /// 
    /// Return None if the request content-type is not application/x-www-form-urlencoded, otherwise return the HashMap in the Option object.
    pub fn decode_body(&self) -> Option<HashMap<String, String>> {
        if self.get_header("Content-Type").unwrap_or("").contains("application/x-www-form-urlencoded") {
            let mut params = HashMap::new();
            for param in self.body.split("&") {
                let parts = param.split("=").collect::<Vec<&str>>();
                params.insert(String::from(parts[0]), String::from(*parts.get(1).unwrap_or(&"")));
            }
            Some(params)
        } else {
            None
        }
    }
}


/// RequestPath struct, used to represent the path of the request.
/// path is the path of the request without the query, and query is the opposite.
#[derive(Debug, Clone)]
pub struct RequestPath {
    path: Vec<String>,
    query: HashMap<String, String>,
}

impl RequestPath {

    /// Create a new RequestPath struct, used when a client make a request.
    pub fn new(path: String) -> Self {
        let url = path.split_once("?").unwrap_or((path.as_str(), ""));
        let path = url.0.split("/").filter(|s| !s.is_empty()).map(|s| String::from(s)).collect::<Vec<String>>();
        let query = url.1.split("&").filter(|s| !s.is_empty()).map(|s| s.split("=").collect::<Vec<&str>>()).map(|v| (String::from(v[0]), String::from(*v.get(1).unwrap_or(&"")))).collect::<HashMap<String, String>>();
        RequestPath {
            path,
            query
        }
    }

    /// Create a new RequestPath struct, used when the server create a new route.
    pub fn new_route(path: String) -> Self {
        let path = path.split("/").filter(|s| !s.is_empty()).map(|s| String::from(s)).collect::<Vec<String>>();
        RequestPath {
            path,
            query: HashMap::with_capacity(0),
        }
    }
    
    /// Return the value of the query parameter, if the query parameter is not found, return None.
    pub fn get_query(&self, key: &str) -> Option<&String> {
        self.query.get(key)
    }

    /// Return the path of the request, without the query.
    pub fn get_path(&self) -> String {
        self.path.join("/")
    }

    /// Return the value of the path parameter at the position index.
    /// 
    /// Return None if the index is out of range.
    /// 
    /// ## Example: 
    /// ```request: /user/profile?id=1
    /// get_path_positon(0) => /user
    /// get_path_positon(1) => /profile
    /// get_path_positon(2) => None```
    pub fn get_path_position(&self, position: usize) -> Option<String> {
        self.path.get(position).map(|s| s.clone())
    }
}

impl Hash for RequestPath {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.path.hash(state);
    }
}

impl PartialEq for RequestPath {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl Eq for RequestPath {}

impl Display for RequestPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "/{}, params: {:?}", self.get_path(), self.query)
    }
}
