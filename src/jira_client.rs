extern crate serde;
extern crate serde_json;

use hyper::Client;
use hyper::client::{Body, Response};
use hyper::header::Headers;
use hyper::status::StatusCode;
use jira_request::*;
use std::io::Read;

pub fn send<R: JIRARequest, T: serde::de::Deserialize>(request: R) -> Result<T, String> {
    let client = Client::new();
    let url = format!("{}{}", &request.base_url(), &request.path());

    let mut res: Response = client.request(request.method(), &url)
        .headers(request.headers().unwrap_or(Headers::new()))
        .body(&request.body().unwrap_or("".to_string()))
        .send()
        .map_err(|e| e.to_string())?;

    let mut body = String::new();
    res.read_to_string(&mut body).map_err(|e| e.to_string())?;
    match res.status {
        StatusCode::Ok => serde_json::from_str::<T>(&body).map_err(|e| e.to_string()),
        _ => Err(body)
    }
}