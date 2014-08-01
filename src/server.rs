use std::io::{Acceptor, BufferedWriter, Listener, TcpListener};

use handler::Handler;
use request::Request;
use response::Response;
use status::BadRequest;


pub struct Server {
    addr: String,
    port: u16,
    handlers: Vec<Box<Handler + Send>>,
}


impl Server {
    pub fn new (addr: &str, port: u16,
                handlers: Option<Vec<Box<Handler + Send>>>) -> Server {
        Server {
            addr: addr.to_string(),
            port: port,
            handlers: match handlers {
                None => Vec::new(),
                Some(handlers) => handlers,
            },
        }
    }

    pub fn add_handler (&mut self, handler: Box<Handler + Send>) {
        self.handlers.push(handler);
    }

    pub fn run (&self) {
        let listener = TcpListener::bind(self.addr.as_slice(), self.port);
        let mut acceptor = listener.listen();

        for stream in acceptor.incoming() {
            match stream {
                Err(_) => {
                    // Connection failed
                    // TODO: ???
                    fail!("Connection failed");
                }
                Ok(stream) => {
                    let handlers = self.handlers.clone();
                    spawn(proc() {
                        let req_stream = stream.clone();
                        let request = Request::from_tcp_stream(req_stream);
                        let mut response = Response::blank();

                        match request {
                            Ok(ref request) => {
                                for handler in handlers.iter() {
                                    let cont = handler.handle(request, &mut response);
                                    if !cont {
                                        break;
                                    }
                                }
                            },
                            Err(_) => response.set_status(BadRequest),
                        }

                        let mut resp_stream = BufferedWriter::new(stream);

                        let status_line = response.get_status_line();
                        match resp_stream.write_str(status_line.as_slice()) {
                            Ok(_) => {},
                            Err(_) => {},
                        }

                        let header_block = response.get_header_block();
                        match resp_stream.write_str(header_block.as_slice()) {
                            Ok(_) => {},
                            Err(_) => {},
                        }

                        match resp_stream.write_str("\r\n") {
                            Ok(_) => {},
                            Err(_) => {},
                        }

                        let body = response.get_body();
                        match resp_stream.write_str(body.as_slice()) {
                            Ok(_) => {},
                            Err(_) => {},
                        }
                    });
                }
            }
        }

        drop(acceptor);
    }
}
