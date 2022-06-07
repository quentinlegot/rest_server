use std::net::TcpStream;
use std::io::Write;
use std::collections::HashMap;

pub struct Response {
    pub status: Status,
    pub headers: HashMap<String, String>,
    pub body: String,
    stream: TcpStream,
}

impl Response {
    pub fn new(stream: TcpStream) -> Self {
        let mut headers = HashMap::new();
        headers.insert(String::from("Content-Type"), String::from("text/html; charset=utf-8"));
        Self {
            status: Status::Ok,
            headers,
            body: String::new(),
            stream,
        }
    }

    pub fn set_header(&mut self, key: String, value: String) {
        self.headers.insert(key, value);
    }

    pub fn remove_header(&mut self, key: String) {
        self.headers.remove(&key);
    }

    pub fn set_status(&mut self, status: Status) {
        self.status = status;
    }

    pub fn set_body(&mut self, body: &str) {
        self.body = body.to_string();
    }

    pub fn send(&mut self) {
        let status_line = format!("HTTP/1.1 {}\r\n", self.status.as_str());
        let mut response = status_line.to_string();
        self.set_header(String::from("Content-Length"), self.body.len().to_string());
        for (key, value) in &self.headers {
            response.push_str(&format!("{}: {}\r\n", key, value));
        }
        response.push_str("\r\n");
        response.push_str(&self.body);
        self.stream.write(response.as_bytes()).unwrap();
        self.stream.flush().unwrap();
    }
}

#[repr(u16)]
pub enum Status {
    Ok = 200,
    NotFound = 404,
}

impl Status {
    fn as_str(&self) -> &'static str {
        match self {
            Status::Ok => "200 OK",
            Status::NotFound => "404 Not Found",
        }
    }
}

impl From::<u16> for Status {
    fn from(status: u16) -> Self {
        match status {
            200 => Status::Ok,
            404 => Status::NotFound,
            _ => panic!("Unknown status code"),
        }
    }
}