mod http;

use http::request::{
    RequestMethod,
    RequestBuilder
};

fn main() {
    let req = RequestBuilder::new()
        .set_method(RequestMethod::POST)
        .set_target(String::from("/"))
        .write_header(String::from("Accept"), String::from("text/html"))
        .write_header(String::from("Accept-Language"), String::from("en-US"))
        .write_body(String::from("{}"))
        .get_request();

    println!("{}", req.to_http_message());
}
