use std::io::File;
use std::os::make_absolute;

use method;
use request::Request;
use response::Response;
use status;


pub type Continue = bool;


pub trait Handler: Clone + Send {
    fn handle (&self, request: &Request, response: &mut Response) -> Continue;

    fn clone_box (&self) -> Box<Handler + Send> {
        box self.clone() as Box<Handler + Send>
    }
}


impl Clone for Box<Handler + Send> {
    fn clone (&self) -> Box<Handler + Send> {
        self.clone_box()
    }
}


#[deriving(Clone)]
pub struct StaticHandler {
    prefix: Path,
    dir: Path,
}

impl StaticHandler {
    pub fn new (prefix: &str, dir: &str) -> StaticHandler {
        assert!(prefix.char_at(0) == '/');
        StaticHandler {
            prefix: PosixPath::new(prefix),
            dir: make_absolute(&Path::new(dir)),
        }
    }

    /// Get absolute file system path for URI
    ///
    /// If the URI doesn't start with the configured prefix, None will
    /// be returned instead of a path.
    fn get_fs_path (&self, uri: &str) -> Option<Path> {
        assert!(uri.char_at(0) == '/');
        let uri = PosixPath::new(uri);
        if self.prefix.is_ancestor_of(&uri) {
            match uri.path_relative_from(&self.prefix) {
                Some(rel_path) => Some(self.dir.join(rel_path)),
                None => None,
            }
        } else {
            None
        }
    }
}

impl Handler for StaticHandler {
    fn handle (&self, request: &Request, response: &mut Response) -> Continue {
        let uri = request.path.as_slice();

        match self.get_fs_path(uri) {
            Some(path) => {
                if !path.exists() {
                    response.set_status(status::NotFound);
                } else {
                    match path.stat() {
                        Ok(stat) => {
                            match request.method {
                                method::HEAD => {
                                    response.set_header("Content-Length", stat.size.to_string().as_slice());
                                }
                                method::GET => {
                                    let mut file = match File::open(&path) {
                                        Ok(file) => file,
                                        Err(_) => {
                                            response.set_status(status::InternalServerError);
                                            return false;
                                        }
                                    };
                                    match file.read_to_string() {
                                        Ok(body) => {
                                            response.set_header("Content-Length", stat.size.to_string().as_slice());
                                            response.set_body(body);
                                        },
                                        Err(_) => {
                                            response.set_status(status::InternalServerError);
                                            return false;
                                        }
                                    }
                                },
                                _ => {
                                    response.set_status(status::MethodNotAllowed);
                                },
                            }
                        },
                        Err(_) => {
                            response.set_status(status::InternalServerError);
                        }
                    }
                }

                false
            },
            None => true,
        }
    }
}
