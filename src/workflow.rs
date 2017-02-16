extern crate toml;

use config::Config;
use std::env;
use std::fs::File;
use std::io::Read;
use std::process;

pub struct Workflow {
    config: Config,
}

impl Workflow {
    pub fn config(&self) -> &Config {
        &self.config
    }
}

pub fn new() -> Workflow {
    let config = match load_config() {
        Ok(c) => c,
        Err(e) => {
            println!("{}", e);
            process::exit(1);
        }
    };
    Workflow {
        config: config,
    }
}

fn load_config() -> Result<Config, String> {
    let mut config_file = env::home_dir().unwrap();
    config_file.push(".jiraconfig");
    let mut file = File::open(&config_file)
        .map_err(|_| "Missing .jiraconfig".to_string())?;

    let mut toml_string = String::new();
    file.read_to_string(&mut toml_string).unwrap();

    toml::from_str::<Config>(&toml_string)
        .map_err(|e| format!("Invalid format {}", e))
}