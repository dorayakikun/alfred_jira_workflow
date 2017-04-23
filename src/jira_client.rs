extern crate serde_json;

use errors::*;
use jira_request::*;
use reqwest;
use reqwest::header::Headers;
use reqwest::Response;
use reqwest::StatusCode;
use std::io::Read;

pub fn send<R: JIRARequest>(request: R) -> Result<R::Response> {
    let client = reqwest::Client::new().unwrap();

    let url = format!("{}{}", &request.base_url(), &request.path());

    let mut res: Response = client.request(request.method(), &url)
        .headers(request.headers().unwrap_or(Headers::new()))
        .json(&request.body().unwrap_or("".to_string()))
        .send()
        .chain_err(|| "Failed to request JIRA")?;

    info!("Status: {}", res.status());
    info!("Headers:\n{}", res.headers());

    ::std::io::copy(&mut res, &mut ::std::io::stdout()).unwrap();

    let mut body = vec![];
    res.read_to_end(&mut body).unwrap();

    match res.status() {
        &StatusCode::Ok => {
            serde_json::from_str::<R::Response>(&String::from_utf8_lossy(&body).into_owned())
                .chain_err(|| "Failed to deserialize to the struct")
        }
        _ => bail!(String::from_utf8_lossy(&body).to_string()),
    }
}