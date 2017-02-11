extern crate toml;
use config::Config;

pub struct Workflow {
    config: Config,
}

pub fn new() -> Workflow {
    let config = load_config();
    Workflow {
        config: config,
    }
}

fn load_config() -> Result<Config, String> {
    let mut config_file: String = env::home_dir().unwrap();
    config_file.push(".jiraconfig");

    match toml::from_str(&config_file) {
        Ok(c) => c,
        Err(_) => Err("Failed load config file")
    }
}