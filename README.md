# rest_server

This program is a minimal REST server framework in rust

## Warning!

This program is still in development

Feature to add:

- [ ] Parsing request header
- [ ] Send response header
- [ ] Only available status code are 200 and 404 but more will be available
- [ ] Don't support authentification natively (won't add support shortly as you can made it yourself when `parsing request header` will be done)
- [ ] Only support GET and POST

I'm only support http/1.1 and won't support 2.0

## Usage example

```rust
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
```

## Contribution

If you have an idea to implement, please post a issue, I'll receive PR about this fonctionnality after I validated it on issues page.
