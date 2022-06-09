/*!
 
A Minimalist multi-threaded REST server framework written in Rust.
 
Create a server with a given port and a given routes.
  
# Example

 ```rust
fn main() {
    let mut app = Server::new();
    app.set_number_of_worker(8);
    app.get(String::from("/"), Box::new(index));
    app.listen(7878);
}

fn index(request: Request, mut response: Response) {
    let content = "Hello";
    println!("{}", request.get_header("User-Agent").unwrap());
    response.set_status(Status::from(418));
    response.set_header(String::from("Content-Type"), String::from("text/plain"));
    response.set_body(content);
    response.send();
}
```

 */
mod threadpool;
pub mod response;
pub mod request;
pub mod method;
pub mod status;
use threadpool::ThreadPool;
use method::Method;
use request::{Request, RequestPath};
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
    /// Store all the registered routes.
    /// The key is the combinaison of the method and the path, the value is the function to call.
    /// 
    /// The usage of a RwLock(Read, write lock) is to avoid thead-safety issues, read is non blocking, write is blocking.
    /// write lock is used to add a new route, read lock is used to get the function to call.
    static ref ROUTING: Arc<RwLock<HashMap<(Method, RequestPath), Box<IFn>>>> = {
        let arc: Arc<RwLock<HashMap<(Method, RequestPath), Box<IFn>>>> = Arc::new(RwLock::new(HashMap::new()));
        arc.write().unwrap().insert((Method::GET, RequestPath::new(String::from("/404.html"))), Box::new(not_found));
        arc
    };
}

/// Default route if no route is found or if client call /404.html
fn not_found(_req: Request, mut res: Response) {
    res.set_status(Status::NotFound);
    res.set_header(String::from("Content-Type"), String::from("text/plain; charset=utf-8"));
    res.set_body("404 Not Found");
    res.send();
}

/// Main struct, start the server and listen on the port given in argument.
/// 
/// number_of_workers is the number of threads used to handle the requests.
/// My advice is to set number_of_workers to the number of logical cores of your CPU.
pub struct Server {
    number_of_workers: usize,
}

impl Server {
    
    /// Create a new Server.
    /// Socket isn't opened yet, you have to call listen() to open it.
    pub fn new() -> Self {
        Self {
            number_of_workers: 4,
        }
    }

    /// add a new GET route to the server with the given path and the given function
    pub fn get(&mut self, path: String, f: Box<IFn>) {
        self.route(Method::GET, path, f);
    }

    /// add a new POST route to the server with the given path and the given function
    pub fn post(&mut self, path: String, f: Box<IFn>) {
        self.route(Method::POST, path, f);
    }

    /// add a new PUT route to the server with the given path and the given function
    pub fn put(&mut self, path: String, f: Box<IFn>) {
        self.route(Method::PUT, path, f);
    }

    /// add a new DELETE route to the server with the given path and the given function 
    pub fn delete(&mut self, path: String, f: Box<IFn>) {
        self.route(Method::DELETE, path, f);
    }

    /// add a new HEAD route to the server with the given path and the given function
    pub fn head(&mut self, path: String, f: Box<IFn>) {
        self.route(Method::HEAD, path, f);
    }

    /// add a new OPTIONS route to the server with the given path and the given function
    pub fn options(&mut self, path: String, f: Box<IFn>) {
        self.route(Method::OPTIONS, path, f);
    }

    /// add a new CONNECT route to the server with the given path and the given function
    pub fn connect(&mut self, path: String, f: Box<IFn>) {
        self.route(Method::CONNECT, path, f);
    }

    /// add a new TRACE route to the server with the given path and the given function
    pub fn trace(&mut self, path: String, f: Box<IFn>) {
        self.route(Method::TRACE, path, f);
    }

    /// add a new PATCH route to the server with the given path and the given function
    pub fn patch(&mut self, path: String, f: Box<IFn>) {
        self.route(Method::PATCH, path, f);
    }

    /// add a new route to the server with the given method, the given path and the given function
    pub fn route(&mut self, method: Method, path: String, f: Box<IFn>) {
        ROUTING.write().unwrap().insert((method, RequestPath::new_route(path)), f);
    }

    /// Set the number of workers used to handle the requests.
    /// The default value is 4.
    /// 
    /// See the documentation of the ThreadPool struct for more information.
    /// 
    /// If you set the number of workers to 0, the program will panic.
    pub fn set_number_of_worker(&mut self, number: usize) {
        assert!(number > 0);
        self.number_of_workers = number;
    }

    /// Open the socket and listen on the given port.
    /// The socket is opened in blocking mode to use least CPU usage possible.
    /// 
    /// Because of that, if you use ctrl+c, the program will not stop immediately, but will wait for the current requests and the next ones to finish.
    /// After, the socket is closed, destructor will be called and the program will stop.
    /// 
    /// port is the port on which the server will listen, if port isn't positive, the program will panic.
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
                    let routing_clone = Arc::clone(&ROUTING);
                    pool.execute(move || {
                        let r = routing_clone.read().unwrap();
                        println!("[REQUEST] {} {}", request.method, request.path);
                        let route = r.get(&(request.method, request.path.clone())).unwrap();
                        route(request, response)
                    });
                    let clone = Arc::clone(&exit);
                    if clone.read().unwrap().load(std::sync::atomic::Ordering::SeqCst) {
                        drop(pool);
                        break;
                    }
                    drop(clone);
                },
                Err(e) => {
                    eprintln!("{}", e);
                },
            }

        }
    }

    /// Handle a connection and return a tuple containing the request and the response usable to send a response to the client.
    /// If a path is not found, the function will return a 404 not found response.
    /// If the method is not supported, the function will return an Result::Err containing the error. (In the future, it will return a 405 method not allowed response)
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
                    Ok((Request::new(Method::GET, RequestPath::new_route(String::from("/404.html")), String::new()), Response::new(stream.try_clone().unwrap())))
                    //Err(format!("No handler found for {} {}", method.0, method.1))
                },
            }
        } else {
            Err(String::from("No method found"))
        }
    }

    /// Construct a request from the given content and the given method and path.
    fn construct_request(&self, content: String, method: Method, path: RequestPath) -> Request {
        request::Request::new(method, path, content)
    }

    /// Construct a response from the given stream.
    /// The stream is used to send the response to the client and is closed when the response is sent.
    fn construct_response(&self, stream: TcpStream) -> Response {
        response::Response::new(stream)
    }
}