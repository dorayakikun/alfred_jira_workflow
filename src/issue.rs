use fields::Fields;

#[derive(Deserialize)]
pub struct Issue {
    pub expand: String,
    pub id: String,
    #[serde(rename = "self")]
    pub url: String,
    pub key: String,
    pub fields: Fields,
}
