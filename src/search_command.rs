extern crate alfred;

use config::Config;
use item::Item;
use item::ItemBuilder;
use jira_api::SearchIssue;
use jira_client;
use search_response::SearchResponse;
use serde_json;
use std::io;

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
        let alfred_items = res.issues
            .into_iter()
            .map(|issue| {
                     ItemBuilder::new()
                         .title(issue.fields.summary)
                         .arg(format!("{}/browse/{}", base_url, issue.key))
                         .build()
                 })
            .collect::<Vec<Item>>();

        serde_json::to_writer_pretty(&mut io::stdout(), &alfred_items).map_err(|e| e.to_string())
    }
}