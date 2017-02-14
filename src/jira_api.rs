use hyper::method::*;
use jira_request::*;

struct SearchIssue {
    base_url: String,
    path: String,
    method: Method,
    parameters: String,
}

impl SearchIssue {
    fn new(base_url: &String, path: &String, parameters: &String) -> SearchIssue {
        SearchIssue {
            base_url: base_url.to_string(),
            path: path.to_string(),
            method: Method::Get,
            parameters: parameters.to_string(),
        }
    }
}

impl JIRARequest for SearchIssue {
    fn base_url(&self) -> &String {
        &self.base_url
    }
    fn path(&self) -> &String {
        &self.path
    }

    fn method(&self) -> &Method {
        &self.method
    }

    fn parameters(&self) -> &String {
        &self.parameters
    }
}

#[cfg(test)]
mod test {
    use hyper::method::Method;
    use super::*;

    #[test]
    fn it_works() {
        let search_issue = SearchIssue::new(
            &"http://example.com".to_string(),
            &"/test".to_string(),
            &"key: value".to_string(),
        );

        assert_eq!("http://example.com", search_issue.base_url());
        assert_eq!("/test", search_issue.path());
        assert_eq!(&Method::Get, search_issue.method());
        assert_eq!("key: value", search_issue.parameters());
    }
}