use super::header::Header;

pub struct Request {
    pub method: String,
    pub target: String,
    pub headers: Vec<Header>,
    pub body: String,
}

impl Request {
    pub fn new() -> Request {
        Request {
            method: String::new(),
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
                headers_string = format!("{}\r\n{}", headers_string, header.to_string());
            }
        }
        
        headers_string
    }
}
