extern crate alfred;

use config::Config;
use jira_api::SearchIssue;
use jira_client;
use search_response::SearchResponse;
use std::io::{self};

pub struct SearchCommand {
    pub config: Config,
}

impl SearchCommand {
    pub fn run(&self, keyword: &String) -> Result<(), String> {
        let jira_search = SearchIssue {
            config: self.config.clone(),
            keyword: keyword.to_string(),
        };
        let res = jira_client::send::<SearchIssue>(jira_search).map_err(|e| e.to_string())?;
        Ok(self.write_alfred_items(self.config.hostname(), res)?)
    }

    fn write_alfred_items(&self, base_url: &String, res: SearchResponse) -> Result<(), String> {
        let alfred_items = res.issues.into_iter()
            .map(|issue| alfred::ItemBuilder::new(issue.fields.summary).arg(format!("{}/browse/{}", base_url, issue.key)).into_item())
            .collect::<Vec<alfred::Item>>();

        alfred::json::write_items(io::stdout(), &alfred_items).map_err(|e| e.to_string())
    }
}