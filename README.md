# rest_server

This program is a minimal REST server framework in rust

## Warning!

This program is still in development

Feature to add:

- [ ] https://github.com/quentinlegot/rest_server/issues/1
- [ ] https://github.com/quentinlegot/rest_server/issues/2
- [ ] Don't support authentification natively (won't add support shortly as you can made it yourself when `parsing request header` will be done)
- [ ] https://github.com/quentinlegot/rest_server/issues/3

I'm only support http/1.1 and won't support 2.0

## Usage example

```rust
fn main() {
    let mut app: Server = Server::new();
    app.set_number_of_worker(8);
    app.get(String::from("/"), Box::new(index));
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
