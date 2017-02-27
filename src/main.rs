extern crate clap;
extern crate hyper;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate toml;

mod config;
mod issue;
mod jira_api;
mod jira_client;
mod jira_request;
mod search_command;
mod search_response;
mod workflow;

use clap::{App, SubCommand};

fn main() {
    let workflow = workflow::new();
    let matches = App::new("jira")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Alfred JIRA Workflow.")
        .subcommand(SubCommand::with_name("Search")
            .about("Search JIRA issues"))
        .get_matches();

    if let Some(_) = matches.subcommand_matches("search") {
        match workflow.seach().run(&"".to_string()) {
            Ok(_) => (),
            Err(e) => {
                println!("{}", e.to_string());
                std::process::exit(1);
            }
        }
        std::process::exit(0);
    };
    std::process::exit(0);
}
