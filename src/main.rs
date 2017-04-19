extern crate clap;
extern crate hyper;
extern crate hyper_native_tls;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate toml;

mod alfred_result;
mod config;
mod fields;
mod issue;
mod item;
mod jira_api;
mod jira_client;
mod jira_request;
mod search_command;
mod search_response;
mod workflow;

use clap::{Arg, App, SubCommand};

fn main() {
    let workflow = workflow::new();
    let matches = App::new("jira")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Alfred JIRA Workflow.")
        .subcommand(SubCommand::with_name("search")
            .about("Search JIRA issues")
            .arg(Arg::with_name("keyword")
                .required(true)))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("search") {
        if let Some(keyword) = matches.value_of("keyword") {
            match workflow.search().run(&keyword.to_string()) {
                Ok(_) => (),
                Err(e) => {
                    println!("{}", e.to_string());
                    std::process::exit(1);
                }
            }
        }
        std::process::exit(0);
    };
    std::process::exit(0);
}
