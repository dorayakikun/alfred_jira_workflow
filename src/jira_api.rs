use config::Config;
use hyper::client::{Body};
use hyper::header::{Authorization, Basic, Headers, Header};
use hyper::method::*;
use jira_request::*;

pub struct SearchIssue {
    pub config: Config,
    pub keyword: String,
}

impl JIRARequest for SearchIssue {
    fn base_url(&self) -> String {
        let base_url = format!("{}", self.config.hostname());
        base_url
    }

    fn path(&self) -> String {
        let path = format!("/rest/api/2/search?jql={}~&maxResults=15", &self.keyword);
        path
    }

    fn method(&self) -> Method {
        Method::Get
    }

    fn headers(&self) -> Option<Headers> {
        let mut headers = Headers::new();
        headers.set(Authorization(Basic {
            username: self.config.username().to_string(),
            password: Some(self.config.password().to_string()) }));
        Some(headers)
    }

    fn body(&self) -> Option<String> {
        None
    }
}

#[cfg(test)]
mod test {
    use config::Config;
    use hyper::method::Method;
    use super::*;

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
        assert_eq!("/rest/api/2/search?jql=keyword~&maxResults=15", &search_issue.path());
        assert_eq!(Method::Get, search_issue.method());
    }
}