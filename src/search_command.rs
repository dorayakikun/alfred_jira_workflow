use alfred_result::AlfredResult;
use config::Config;
use errors::*;
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
    pub fn run(&self, keyword: &String) -> Result<()> {
        let jira_search = SearchIssue {
            config: self.config.clone(),
            keyword: keyword.to_string(),
        };
        let res = jira_client::send::<SearchIssue>(jira_search).chain_err(|| "Failed to call search API".to_string())?;
        Ok(self.write_alfred_items(self.config.hostname(), res)?)
    }

    fn write_alfred_items(&self, base_url: &String, res: SearchResponse) -> Result<()> {
        let alfred_items = res.issues
            .into_iter()
            .map(|issue| {
                     ItemBuilder::new()
                         .title(issue.fields.summary)
                         .arg(format!("{}/browse/{}", base_url, issue.key))
                         .build()
                 })
            .collect::<Vec<Item>>();

        let alfred_result = AlfredResult { items: alfred_items };
        serde_json::to_writer_pretty(&mut io::stdout(), &alfred_result).chain_err(|| "Failed to the serialize pretty JSON")
    }
}