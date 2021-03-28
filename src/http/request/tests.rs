use crate::http::request::{
    RequestMethod,
    RequestBuilder,
};

#[test]
fn request_test() {
    let method = RequestMethod::POST;
    let target = ("/index.html").to_string();
    let body = ("{}").to_string();

    let req = RequestBuilder::new()
        .set_method(method)
        .set_target(target)
        .write_header(("a").to_string(), ("b").to_string())
        .write_header(("c").to_string(), ("d").to_string())
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
