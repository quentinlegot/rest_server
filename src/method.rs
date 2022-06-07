use std::fmt::{Display, Formatter};

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum Method {
    GET,
    POST
}

impl Method {
    pub fn as_str(&self) -> &'static str {
        match self {
            Method::GET => "GET",
            Method::POST => "POST"
        }
    }

    pub fn parse_method(content: Option<&&str>) -> Result<(Method, String), &'static str> {
        match content {
            Some(s) => {
                let el: Vec<&str> = s.split(" ").collect();
                if let Some(s) = el.get(0) {
                    match s {
                        &"GET" => { Ok((Method::GET, Self::parse_path(el.get(1)))) },
                        &"POST" => { Ok((Method::GET, Self::parse_path(el.get(1)))) },
                        _ => { Err("Can't parse method") }
                    }
                } else {
                    Err("Can't parse method")
                }
            },
            None => Err("Can't parse method")
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