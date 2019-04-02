use reqwest::header::HeaderMap;
use reqwest::Method;
use serde::de::DeserializeOwned;

pub trait JIRARequest {
    type Response: DeserializeOwned;

    fn base_url(&self) -> String;
    fn path(&self) -> String;
    fn method(&self) -> Method;
    fn headers(&self) -> Option<HeaderMap>;
    fn body(&self) -> Option<String>;
}
