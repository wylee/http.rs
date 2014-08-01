use header::{Headers, Header};
use status::{Status, OK};


pub struct Response {
    version: String,
    status: Status,
    headers: Headers,
    body: String,
}


impl Response {
    pub fn new (status: Status, headers: Headers, body: String) -> Response {
        Response {
            version: "1.1".to_string(),
            status: status,
            headers: headers,
            body: body,
        }
    }

    pub fn blank () -> Response {
        Response::new(OK, Headers::new(), "".to_string())
    }

    pub fn get_status_line (&self) -> String {
        format!("HTTP/{} {}\r\n", self.version, self.status)
    }

    pub fn get_header_block (&self) -> String {
        let mut header_block = String::new();
        for (_, header) in self.headers.iter() {
            header_block.push_str(header.name.as_slice());
            header_block.push_str(": ");
            header_block.push_str(header.value.as_slice());
            header_block.push_str("\r\n");
        }
        header_block
    }

    pub fn get_body (&self) -> String {
        format!("{}", self.body)
    }

    pub fn set_body (&mut self, body: String) {
        self.body = body
    }

    pub fn set_status (&mut self, status: Status) {
        self.status = status;
    }

    pub fn get_header<'a> (&'a self, name: &str) -> Option<&'a Header> {
        self.headers.find(&name.to_string())
    }

    pub fn set_header (&mut self, name: &str, value: &str) -> bool {
        self.headers.insert(name, value)
    }
}
