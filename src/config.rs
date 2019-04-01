#[derive(Clone, Deserialize)]
pub struct Config {
    pub hostname: String,
    pub username: String,
    pub password: String,
}

impl Config {
    pub fn hostname(&self) -> &String {
        &self.hostname
    }
    pub fn username(&self) -> &String {
        &self.username
    }
    pub fn password(&self) -> &String {
        &self.password
    }
}
