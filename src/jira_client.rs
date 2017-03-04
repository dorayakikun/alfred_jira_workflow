extern crate serde_json;

use hyper::Client;
use hyper::client::{Response};
use hyper::header::Headers;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use hyper::status::StatusCode;
use jira_request::*;
use std::io::Read;

pub fn send<R: JIRARequest>(request: R) -> Result<R::Response, String> {
    let tls = NativeTlsClient::new().unwrap();
    let connector = HttpsConnector::new(tls);
    let client = Client::with_connector(connector);

    let url = format!("{}{}", &request.base_url(), &request.path());

    let mut res: Response = client.request(request.method(), &url)
        .headers(request.headers().unwrap_or(Headers::new()))
        .body(&request.body().unwrap_or("".to_string()))
        .send()
        .map_err(|e| e.to_string())?;

    let mut body = vec![];
    res.read_to_end(&mut body).unwrap();

    match res.status {
        StatusCode::Ok => serde_json::from_str::<R::Response>(&String::from_utf8_lossy(&body)).map_err(|e| e.to_string()),
        _ => Err(String::from_utf8_lossy(&body).to_string()),
    }
}