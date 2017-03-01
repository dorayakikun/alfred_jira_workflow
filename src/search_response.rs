use issue::Issue;

#[derive(Deserialize)]
pub struct SearchResponse {
    pub expand: String,
    #[serde(rename = "startAt")]
    pub start_at: i64,
    #[serde(rename = "maxResult")]
    pub max_result: i64,
    pub issues: Vec<Issue>,
}