use config::Config;
use base64::encode;
use jira_request::*;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Method;
use search_response::SearchResponse;

pub struct SearchIssue {
    pub config: Config,
    pub keyword: String,
}

impl JIRARequest for SearchIssue {
    type Response = SearchResponse;

    fn base_url(&self) -> String {
        self.config.hostname().to_string()
    }

    fn path(&self) -> String {
        let path = format!(
            "/rest/api/2/search?jql=text~{}&maxResults=50",
            &self.keyword
        );
        path
    }

    fn method(&self) -> Method {
        Method::GET
    }

    fn headers(&self) -> Option<HeaderMap> {
        let mut header_map = HeaderMap::new();

        header_map.insert(CONTENT_TYPE, HeaderValue::from_static("application/x-www-form-urlencoded"));
        let auth = format!("{}:{}", self.config.username(), self.config.password());
        let header_value = format!("Basic {}", encode(&auth));
        header_map.insert(AUTHORIZATION, HeaderValue::from_str(&header_value).unwrap());// FIXME

        Some(header_map)
    }

    fn body(&self) -> Option<String> {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use config::Config;
    use reqwest::Method;

    #[test]
    fn it_works() {
        let search_issue = SearchIssue {
            config: Config {
                hostname: "http://localhost".to_string(),
                username: "test".to_string(),
                password: "pass".to_string(),
            },
            keyword: "keyword".to_string(),
        };

        assert_eq!("http://localhost", &search_issue.base_url());
        assert_eq!(
            "/rest/api/2/search?jql=text~keyword&maxResults=50",
            &search_issue.path()
        );
        assert_eq!(Method::GET, search_issue.method());
    }
}
