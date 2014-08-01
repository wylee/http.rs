use std::collections::HashMap;
use std::collections::hashmap::Entries;
use std::from_str::FromStr;


pub struct Headers {
    map: HashMap<String, Header>,
}

impl Headers {
    pub fn new () -> Headers {
        let headers = Headers {
            map: HashMap::new(),
        };
        headers
    }

    pub fn insert (&mut self, name: &str, value: &str) -> bool {
        let key = name.to_string();
        let name = name.to_string();
        let value = value.to_string();
        let header = Header { name: name, value: value };
        self.map.insert(key, header)
    }

    pub fn insert_line (&mut self, line: &str) -> Option<Header> {
        from_str::<Header>(line)
    }

    pub fn find<'a> (&'a self, name: &String) -> Option<&'a Header> {
        self.map.find(name)
    }

    pub fn content_length (&self) -> Result<Option<uint>, &'static str> {
        let h = self.map.find(&"Content-Length".to_string());
        match h {
            None => Ok(None),
            Some(h) => {
                match from_str::<uint>(h.value.as_slice()) {
                    Some(length) => Ok(Some(length)),
                    None => Err("Bad Content-Length header value"),
                }
            }
        }
    }

    pub fn iter<'a> (&'a self) -> Entries<'a, String, Header> {
        self.map.iter()
    }
}


pub struct Header {
    pub name: String,
    pub value: String,
}

impl FromStr for Header {
    fn from_str (s: &str) -> Option<Header> {
        let parts: Vec<&str> = s.splitn(':', 1)
                                .map(|s| { s.trim() })
                                .filter(|s| { s.len() > 0 })
                                .collect();
        match parts.as_slice() {
            [name, value] => {
                Some(Header {
                    name: name.to_string(),
                    value: value.to_string(),
                })
            },
            _ => None,
        }
    }
}
