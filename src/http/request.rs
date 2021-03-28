use std::fmt;
use super::header::Header;

#[derive(Debug)]
#[allow(dead_code)]
pub enum RequestMethod {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH
}

impl fmt::Display for RequestMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Request {
    pub method: RequestMethod,
    pub target: String,
    pub headers: Vec<Header>,
    pub body: String,
}

impl Request {
    pub fn new(method: RequestMethod) -> Request {
        Request {
            method,
            target: String::new(),
            headers: Vec::new(),
            body: String::new(),
        }   
    }

    pub fn to_http_message(&self) -> String {
        format!(
            "\
                {} {} HTTP/1.1\r\n\
                {}\r\n\
                \r\n\
                {}\
            ",
            self.method,
            self.target,
            self.headers_str(),
            self.body
        )
    }

    fn headers_str(&self) -> String {
        let mut headers_string = String::new();

        for header in &self.headers {
            if headers_string == String::new() {
                headers_string = header.to_string();
            } else {
                headers_string = format!("{}\r\n{}", headers_string, header);
            }
        }
        
        headers_string
    }
}
