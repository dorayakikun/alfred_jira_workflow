extern crate toml;

use config::Config;
use errors::*;
use search_command::SearchCommand;
use std::fs::File;
use std::io::Read;
use std::process;

pub struct Workflow {
    search: SearchCommand,
}

impl Workflow {
    pub fn search(&self) -> &SearchCommand {
        &self.search
    }
}

pub fn new() -> Workflow {
    let config = match load_config() {
        Ok(c) => c,
        Err(e) => {
            error!("{}", e);
            process::exit(1);
        }
    };
    dbg!(&config);
    Workflow {
        search: SearchCommand { config },
    }
}

fn load_config() -> Result<Config> {
    let mut config_file = dirs::home_dir().unwrap();
    config_file.push(".jiraconfig");
    let mut file = File::open(&config_file).chain_err(|| "Missing .jiraconfig".to_string())?;

    let mut toml_string = String::new();
    file.read_to_string(&mut toml_string).unwrap();

    toml::from_str::<Config>(&toml_string).chain_err(|| "Invalid format {}".to_string())
}
