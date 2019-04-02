use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Fields {
    pub summary: String,
}
