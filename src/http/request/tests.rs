use crate::http::request::{
    RequestMethod,
    RequestBuilder,
};

macro_rules! to_str {
    ($a:expr) => {
        {
            String::from($a)
        }
    };
}

#[test]
fn request_test() {
    let method = RequestMethod::POST;
    let target = to_str!("/index.html");
    let body = to_str!("{}");

    let req = RequestBuilder::new()
        .set_method(method)
        .set_target(target)
        .write_header(to_str!("a"), to_str!("b"))
        .write_header(to_str!("c"), to_str!("d"))
        .write_body(body)
        .get_request();

    assert_eq!(req.to_http_message(), "\
        POST /index.html HTTP/1.1\r\n\
        a: b\r\n\
        c: d\r\n\
        \r\n\
        {}\
    ")
}
