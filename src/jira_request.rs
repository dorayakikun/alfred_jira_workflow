use reqwest::header::Headers;
use reqwest::Method;
use serde;

pub trait JIRARequest {
    type Response: serde::de::Deserialize;

    fn base_url(&self) -> String;
    fn path(&self) -> String;
    fn method(&self) -> Method;
    fn headers(&self) -> Option<Headers>;
    fn body(&self) -> Option<String>;
}
