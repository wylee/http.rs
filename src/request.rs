use std::io::{BufferedReader, TcpStream};

use header::Headers;
use method::Method;


pub struct Request {
    pub method: Method,
    pub path: String,
    headers: Headers,
    body: Option<String>,
}


impl Request {
    pub fn from_tcp_stream (stream: TcpStream) -> Result<Request, &'static str> {
        let mut stream = BufferedReader::new(stream);

        let (method, path) = match stream.read_line() {
            Ok(line) => {
                match Request::parse_request_line(line) {
                    Some((method, path)) => (method, path),
                    None => return Err("Could not parse request line"),
                }
            },
            Err(_) => return Err("Could not read request line"),
        };

        let mut headers = Headers::new();
        loop {
            match stream.read_line() {
                Ok(line) => {
                    let line = line.as_slice().trim();
                    match line {
                        "" => break,
                        line => {
                            match headers.insert_line(line) {
                                Some(_) => (),
                                None => return Err("Could not parse header line"),
                            }
                        }
                    }
                }
                Err(_) => return Err("Could not read header block from TCP stream"),
            }
        }

        let body = match headers.content_length() {
            Ok(None) => None,
            Ok(Some(length)) => {
                match stream.read_exact(length) {
                    Ok(body) => {
                        match String::from_utf8(body) {
                            Ok(body) => Some(body),
                            Err(_) => return Err("Error reading body as UTF-8"),
                        }
                    },
                    Err(_) => return Err("Could not read body from TCP Stream"),
                }
            },
            Err(e) => return Err(e),
        };

        Ok(Request {
            method: method,
            path: path,
            headers: headers,
            body: body,
        })
    }

    fn parse_request_line (line: String) -> Option<(Method, String)> {
        let parts: Vec<&str> = line.as_slice()
                                   .splitn(' ', 2)
                                   .map(|s| { s.trim() })
                                   .filter(|s| { s.len() > 0 })
                                   .collect();

        let (method, path) = match parts.as_slice() {
            [method, path] => (method, path),
            [method, path, version] => (method, path),
            [..] => return None,
        };

        match from_str::<Method>(method) {
            Some(method) => Some((method, String::from_str(path))),
            None => None,
        }
    }
}
