use hyper::Client;
use hyper::client::{RequestBuilder};
use hyper::header::{Headers};
use hyper::method::{Method};

pub trait JIRARequest {
    fn base_url(&self) -> String;
    fn path(&self) -> String;
    fn method(&self) -> Method;
    fn headers(&self) -> Option<Headers>;
    fn body(&self) -> Option<String>;
}