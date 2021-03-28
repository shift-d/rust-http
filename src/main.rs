mod http;

use http::request::Request;
use http::header::Header;

fn main() {
    let mut req = Request::new();
    req.method = String::from("GET");
    req.headers.push(Header::new("Content-Type", "application/json"));
    req.headers.push(Header::new("Content-Length", "16"));
    req.body = String::from("{\"key\": \"value\"}");

    println!("{}", req.to_http_message());
}
