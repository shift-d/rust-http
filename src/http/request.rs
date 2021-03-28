#[cfg(test)]
mod tests;

use std::fmt;
use crate::http::header::Header;

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
    PATCH,
    UNKNOWN,
}

impl fmt::Display for RequestMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Request {
    method: RequestMethod,
    target: String,
    headers: Vec<Header>,
    body: String,
}

impl Default for Request {
    fn default() -> Self {
        Self::new()
    }
}

impl Request {
    pub fn new() -> Request {
        Request {
            method: RequestMethod::UNKNOWN,
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

pub struct RequestBuilder {
    request: Request,
}

impl Default for RequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl RequestBuilder {
    pub fn new() -> RequestBuilder {
        RequestBuilder {
            request: Request::new()
        }
    }

    pub fn set_method(mut self, method: RequestMethod) -> Self {
        self.request.method = method;
        self
    }

    pub fn set_target(mut self, target: String) -> Self {
        self.request.target = target;
        self
    }

    pub fn write_header(mut self, name: String, value: String) -> Self {
        self.request.headers.push(Header::new(name, value));
        self
    }

    pub fn write_body(mut self, body: String) -> Self {
        self.request.body = body;
        self
    }

    pub fn get_request(self) -> Request {
        self.request
    }
}
