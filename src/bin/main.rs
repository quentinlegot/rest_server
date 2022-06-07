use rest_server::response::Response;
use rest_server::response::Status;
use rest_server::request::Request;
use rest_server::Server;
use std::thread;
use std::time::Duration;

fn main() {
    let mut app: Server = Server::new();
    app.set_number_of_worker(8);
    app.get(String::from("/"), Box::new(index));
    app.get(String::from("/sleep"), Box::new(sleep));
    app.listen(7878);
    println!("Shutting down.")
}

fn index(_request: Request, mut response: Response) {
    let content = "Hello";
    response.set_status(Status::from(200));
    response.set_body(content);
    response.send();
}

fn sleep(_request: Request, mut response: Response){
    thread::sleep(Duration::from_secs(5));
    let content = "Sleep";
    response.set_status(Status::from(200));
    response.set_body(content);
    response.send();
}
