use hyper::Client;
use hyper::client::{RequestBuilder};
use hyper::header::{ContentType};
use hyper::method::*;

pub trait JIRARequest {
    fn base_url(&self) -> &String;
    fn path(&self) -> &String;
    fn method(&self) -> &Method;
    fn parameters(&self) -> &String;
}