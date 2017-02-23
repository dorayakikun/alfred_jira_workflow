extern crate alfred;

use config::Config;
use issue::Issue;
use jira_api::SearchIssue;
use jira_client;

struct SearchCommand {
    pub config: Config,
}

impl SearchCommand {
    pub fn run(&self, keyword: &String) -> Result<String, String> {
        let jira_search = SearchIssue{
            config: self.config.clone(),
            keyword: keyword.to_string(),
        };
        let issues = jira_client::send::<SearchIssue, Issue>(jira_search).map_err(|e| e.to_string())?;
        Ok("".to_string())
    }
}