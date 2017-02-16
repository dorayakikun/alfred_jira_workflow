use hyper::client::{Body};
use hyper::header::{Authorization,Basic,Headers,Header};
use hyper::method::*;
use jira_request::*;

struct SearchIssue {
    base_url: String,
    path: String,
    keyword: String,
}

impl JIRARequest for SearchIssue {
    fn base_url(&self) -> String {
        self.base_url.clone()
    }
    fn path(&self) -> String {
        let path = format!("{}/{}", &self.path, &self.keyword);
        path
    }

    fn method(&self) -> Method {
        Method::Get
    }

    fn headers(&self) -> Option<Headers> {
        let mut headers = Headers::new();
        // TODO configからユーザ名とる
        headers.set(Authorization(Basic{username: "".to_string(), password: Some("".to_string())}));
        Some(headers)
    }

    fn body(&self) -> Option<String> {
        None
    }
}

#[cfg(test)]
mod test {
    use hyper::method::Method;
    use super::*;

    #[test]
    fn it_works() {
        let search_issue = SearchIssue {
            base_url: "http://example.com".to_string(),
            path: "/test".to_string(),
            keyword: "keyword".to_string(),
        };

        assert_eq!("http://example.com", &search_issue.base_url());
        assert_eq!("/test/keyword", &search_issue.path());
        assert_eq!(Method::Get, search_issue.method());
    }
}