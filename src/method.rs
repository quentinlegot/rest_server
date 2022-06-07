use std::fmt::{Display, Formatter};

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    OPTIONS,
    CONNECT,
    TRACE,
    PATCH
}

impl Method {

    pub fn as_str(&self) -> &'static str {
        match self {
            Method::GET => "GET",
            Method::POST => "POST",
            Method::PUT => "PUT",
            Method::DELETE => "DELETE",
            Method::HEAD => "HEAD",
            Method::OPTIONS => "OPTIONS",
            Method::CONNECT => "CONNECT",
            Method::TRACE => "TRACE",
            Method::PATCH => "PATCH"
        }
    }

    pub fn from_str(s: &str) -> Option<Method> {
        match s {
            "GET" => Some(Method::GET),
            "POST" => Some(Method::POST),
            "PUT" => Some(Method::PUT),
            "DELETE" => Some(Method::DELETE),
            "HEAD" => Some(Method::HEAD),
            "OPTIONS" => Some(Method::OPTIONS),
            "CONNECT" => Some(Method::CONNECT),
            "TRACE" => Some(Method::TRACE),
            "PATCH" => Some(Method::PATCH),
            _ => None
        }
    }

    pub fn parse_method(content: Option<&&str>) -> Result<(Method, String), &'static str> {
        match content {
            Some(s) => {
                let el = s.split(" ").collect::<Vec<&str>>();
                if let Some(s) = el.get(0) {
                    match Self::from_str(s) {
                        Some(method) => {
                            Ok((method, Self::parse_path(el.get(1))))
                        },
                        None => {
                            Err("Can't parse method: Invalid method")
                        }
                    }
                } else {
                    Err("Can't parse method: no method")
                }
            },
            None => Err("Can't parse method: no content")
        }
    }

    pub fn parse_path(path: Option<&&str>) -> String {
        if let Some(s) = path {
            return String::from(*s);
        } else {
            return String::from("/");
        }
    }
}

impl Display for Method {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}