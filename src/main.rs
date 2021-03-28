mod http;

use http::request::{
    Request,
    RequestMethod
};
use http::header::Header;

fn main() {
    let mut req = Request::new(RequestMethod::POST);
    req.target = String::from("/");
    req.headers.push(Header::new("Content-Type", "application/json"));
    req.headers.push(Header::new("Content-Length", "16"));
    req.body = String::from("{\"key\": \"value\"}");

    println!("{}", req.to_http_message());
}
