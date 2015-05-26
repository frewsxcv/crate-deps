use std::env;
use std::fs;

extern crate git2;
extern crate tiny_http;


static INDEX_GIT_URL: &'static str = "https://github.com/rust-lang/crates.io-index";
static INDEX_LOCAL_PATH: &'static str = "crates.io-index";

fn main() {
    if fs::metadata(INDEX_LOCAL_PATH).is_err() {
        println!("Cloning crates.io-index");
        git2::Repository::clone(INDEX_GIT_URL, INDEX_LOCAL_PATH).unwrap();
    }

    let port = match env::var("PORT") {
        Ok(p) => p.parse::<u16>().unwrap(),
        Err(..) => 8000,
    };

    let server = tiny_http::ServerBuilder::new().with_port(port).build().unwrap();

    println!("Server listening on port {}", port);
    for request in server.incoming_requests() {
        println!("received request! method: {:?}, url: {:?}, headers: {:?}",
            request.get_method(),
            request.get_url(),
            request.get_headers()
        );

        let response = tiny_http::Response::from_string("hello world".to_string());
        request.respond(response);
    }
}
