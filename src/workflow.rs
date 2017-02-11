extern crate toml;
use config::Config;
use std::env;
use std::fs::File;
use std::io::Read;

pub struct Workflow {
    config: Config,
}

pub fn new() -> Workflow {
    let config = load_config().unwrap();
    Workflow {
        config: config,
    }
}

fn load_config() -> Result<Config, String> {
    let mut config_file = env::home_dir().unwrap();
    config_file.push(".jiraconfig");
    let mut file = File::open(&config_file).expect("open config file");

    let mut toml_string = String::new();
    file.read_to_string(&mut toml_string).unwrap();

    match toml::from_str(&toml_string) {
        Ok(c) => c,
        Err(_) => Err("Failed load config file".to_string())
    }
}