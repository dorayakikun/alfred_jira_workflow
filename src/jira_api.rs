use jira_request::*;

struct SearchIssue {
    base_url: String,
    path: String,
    method: String,
    parameters: String,
}

impl JIRARequest for SearchIssue {
    fn base_url(&self) -> &String {
        &self.base_url
    }
    fn path(&self) -> &String {
        &self.path
    }

    fn method(&self) -> &String {
        &self.method
    }

    fn parameters(&self) -> &String {
        &self.parameters
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let search_issue = SearchIssue {
            base_url: "http://example.com".to_string(),
            path: "/test".to_string(),
            method: "GET".to_string(),
            parameters: "key: value".to_string(),
        };

        assert_eq!("http://example.com", search_issue.base_url());
        assert_eq!("/test", search_issue.path());
        assert_eq!("GET", search_issue.method());
        assert_eq!("key: value", search_issue.parameters());
    }
}