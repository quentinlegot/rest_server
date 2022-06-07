mod threadpool;
pub mod response;
pub mod request;
pub mod method;
pub mod status;
use threadpool::ThreadPool;
use method::Method;
use request::Request;
use response::Response;
use status::Status;
use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::str;
use std::sync::{Arc, RwLock};
use std::sync::atomic::AtomicBool;
use ctrlc;

#[macro_use]
extern crate lazy_static;

type IFn = dyn Fn(Request, Response) + Send + 'static + Sync;

lazy_static! {
    static ref ROUTING: Arc<RwLock<HashMap<(Method, String), Box<IFn>>>> = {
        let arc: Arc<RwLock<HashMap<(Method, String), Box<IFn>>>> = Arc::new(RwLock::new(HashMap::new()));
        arc.write().unwrap().insert((Method::GET, String::from("/404.html")), Box::new(not_found));
        arc
    };
}


fn not_found(_req: Request, mut res: Response) {
    res.set_status(Status::NotFound);
    res.set_header(String::from("Content-Type"), String::from("text/plain; charset=utf-8"));
    res.set_body("404 Not Found");
    res.send();
}

pub struct Server {
    number_of_workers: usize,
}

impl Server {

    pub fn new() -> Self {
        Self {
            number_of_workers: 4,
        }
    }

    pub fn get(&mut self, path: String, f: Box<IFn>) {
        self.route(Method::GET, path, f);
    }

    pub fn post(&mut self, path: String, f: Box<IFn>) {
        self.route(Method::POST, path, f);
    }

    pub fn put(&mut self, path: String, f: Box<IFn>) {
        self.route(Method::PUT, path, f);
    }

    pub fn delete(&mut self, path: String, f: Box<IFn>) {
        self.route(Method::DELETE, path, f);
    }

    pub fn head(&mut self, path: String, f: Box<IFn>) {
        self.route(Method::HEAD, path, f);
    }

    pub fn options(&mut self, path: String, f: Box<IFn>) {
        self.route(Method::OPTIONS, path, f);
    }

    pub fn connect(&mut self, path: String, f: Box<IFn>) {
        self.route(Method::CONNECT, path, f);
    }

    pub fn trace(&mut self, path: String, f: Box<IFn>) {
        self.route(Method::TRACE, path, f);
    }

    pub fn patch(&mut self, path: String, f: Box<IFn>) {
        self.route(Method::PATCH, path, f);
    }

    pub fn route(&mut self, method: Method, path: String, f: Box<IFn>) {
        ROUTING.write().unwrap().insert((method, path), f);
    }

    pub fn set_number_of_worker(&mut self, number: usize) {
        assert!(number > 0);
        self.number_of_workers = number;
    }

    pub fn listen(&mut self, port: u32) {
        assert!(port > 0);
        let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).expect("Could not bind to port");
        let pool = ThreadPool::new(self.number_of_workers);
        let exit = Arc::new(RwLock::new(AtomicBool::new(false)));
        let exit_clone = Arc::clone(&exit);

        ctrlc::set_handler(move || {
            // We run like this because we want destructors to run before leaving the program
            println!("Shutting down... (shutdown down sequence will start when next request is received and after all workers are done)");
            exit_clone.write().unwrap().store(true, std::sync::atomic::Ordering::SeqCst);
        }).expect("Error setting Ctrl-C handler");

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            let ifn = self.handle_connection(stream.try_clone().unwrap());
            match ifn {
                Ok((request, response)) => {
                    let clone = Arc::clone(&exit);
                    if clone.read().unwrap().load(std::sync::atomic::Ordering::SeqCst) {
                        break;
                    }
                    drop(clone);
                    let routing_clone = Arc::clone(&ROUTING);
                    pool.execute(move || {
                        let r = routing_clone.read().unwrap();
                        let route = r.get(&(request.method, request.path.clone())).unwrap();
                        println!("[REQUEST] {} {}", request.method, request.path);
                        route(request, response)
                    })
                },
                Err(e) => {
                    eprintln!("{}", e);
                    drop(pool);
                    break;
                },
            }

        }
    }


    fn handle_connection(&self, mut stream: TcpStream) -> Result<(Request, Response), String> {
        let buffer = &mut [0; 1024];
        stream.read(buffer).unwrap();
        let s = match str::from_utf8(buffer) {
            Ok(v) => v,
            Err(e) => return Err(format!("Cannot convert to str {}", e)),
        };
        let content = String::from(s);
        let s = content.split("\r\n").collect::<Vec<&str>>();
        return if let Ok(method) = Method::parse_method(s.get(0)) {
            return match ROUTING.read().unwrap().contains_key(&method) {
                true => {
                    let request = self.construct_request(content, method.0, method.1);
                    let response = self.construct_response(stream.try_clone().unwrap());
                    Ok((request, response))
                },
                false => {
                    Ok((Request::new(Method::GET, String::from("/404.html"), String::new()), Response::new(stream.try_clone().unwrap())))
                    //Err(format!("No handler found for {} {}", method.0, method.1))
                },
            }
        } else {
            Err(String::from("No method found"))
        }
    }

    fn construct_request(&self, content: String, method: Method, path: String) -> Request {
        request::Request::new(method, path, content)
    }

    fn construct_response(&self, stream: TcpStream) -> Response {
        response::Response::new(stream)
    }
}