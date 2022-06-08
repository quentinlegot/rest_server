use std::net::TcpStream;
use std::io::Write;
use std::collections::HashMap;
use crate::status::Status;

/// Response struct, used to send a response to the client.
/// The response is sent by calling the send() method.
pub struct Response {
    pub status: Status,
    pub headers: HashMap<String, String>,
    pub body: String,
    stream: TcpStream,
}

impl Response {

    /// Create a new Response struct
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

    /// Add / Replace a header to the response
    /// If the header already exists, it will be replaced, otherwise it will be added.
    pub fn set_header(&mut self, key: String, value: String) {
        self.headers.insert(key, value);
    }

    /// Remove a header from the response
    /// If the header doesn't exist, nothing will happen.
    pub fn remove_header(&mut self, key: String) {
        self.headers.remove(&key);
    }

    /// Set the status of the response (200, 404, etc.) using the Status enum
    pub fn set_status(&mut self, status: Status) {
        self.status = status;
    }


    /// Set the body of the response
    /// The body will be sent as a string.
    pub fn set_body(&mut self, body: &str) {
        self.body = body.to_string();
    }

    /// Send a HTTP/1.1 response to the client
    /// The stream is closed after the response is sent, so this method should be called only once and at the end of your function.
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
