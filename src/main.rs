extern crate clap;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate toml;

mod config;
mod workflow;

use clap::{App, SubCommand};

fn main() {
    let _ = workflow::new();
    let matches = App::new("jira")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Alfred JIRA Workflow.")
        .subcommand(SubCommand::with_name("Search")
            .about("Search JIRA issues"))
        .get_matches();
}
