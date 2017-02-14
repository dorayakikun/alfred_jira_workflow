extern crate serde;
extern crate serde_json;

use hyper::Client;
use hyper::client::Response;
use hyper::status::StatusCode;
use jira_request::*;
use std::io::Read;

pub fn send<R: JIRARequest, T: serde::de::Deserialize>(request: R) -> Result<T, String> {
    let client = Client::new();
    let mut res: Response = client.request(request.method().clone(), "")
        .send()
        .map_err(|e| e.to_string())?;


    match res.status {
        StatusCode::Ok => {
            let mut body = String::new();
            res.read_to_string(&mut body).map_err(|e| e.to_string())?;
            serde_json::from_str::<T>(&body).map_err(|e| e.to_string())
        },
        _ => Err("".to_string())
    }
}