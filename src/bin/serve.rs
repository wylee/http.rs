#![feature(macro_rules)]

extern crate http;

use http::{Server,Handler,StaticHandler};


fn main () {
    let mut server = Server::new("127.0.0.1", 8080, None);
    server.add_handler(box StaticHandler::new("/", ".") as Box<Handler + Send>);
    server.run();
}
