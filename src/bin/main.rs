use rest_server::response::Response;
use rest_server::status::Status;
use rest_server::request::Request;
use rest_server::Server;
use std::thread;
use std::time::Duration;

fn main() {
    let mut app: Server = Server::new();
    app.set_number_of_worker(8);
    app.get(String::from("/"), Box::new(index));
    app.post(String::from("/"), Box::new(index_post));
    app.patch(String::from("/"), Box::new(index_patch));
    app.get(String::from("/sleep"), Box::new(sleep));
    app.delete(String::from("/sleep"), Box::new(sleep_delete));
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

fn index_post(request: Request, mut response: Response) {
    let content = "Hello post";
    println!("{}", request.get_header("User-Agent").unwrap());
    response.set_status(Status::from(200));
    response.set_header(String::from("Content-Type"), String::from("text/plain"));
    response.set_body(content);
    response.send();
}

fn index_patch(request: Request, mut response: Response) {
    let content = "Hello patch";
    println!("{}", request.get_header("User-Agent").unwrap());
    response.set_status(Status::from(200));
    response.set_header(String::from("Content-Type"), String::from("text/plain"));
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

fn sleep_delete(_request: Request, mut response: Response){
    thread::sleep(Duration::from_secs(5));
    let content = "Sleep delete";
    response.set_status(Status::from(200));
    response.set_body(content);
    response.send();
}
