# rest_server

This program is a minimal REST server framework in rust

## Warning!

This program is still in development

Feature to add:

- [X] [Parsing request header and send response header](https://github.com/quentinlegot/rest_server/issues/1)
- [X] [Add more status code than 200 and 404](https://github.com/quentinlegot/rest_server/issues/2)
- [ ] Don't support authentification natively (won't add support shortly as you can made it yourself when https://github.com/quentinlegot/rest_server/issues/1 will be done)
- [X] [Support other method than get and post](https://github.com/quentinlegot/rest_server/issues/3)

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
