use reqwest::header::HeaderMap;
use reqwest::Method;
use search_response::SearchResponse;

pub trait JIRARequest {
    type Response: Deserialize<'de>;

    fn base_url(&self) -> String;
    fn path(&self) -> String;
    fn method(&self) -> Method;
    fn headers(&self) -> Option<HeaderMap>;
    fn body(&self) -> Option<String>;
}
