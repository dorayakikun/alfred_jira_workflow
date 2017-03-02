use issue::Issue;

#[derive(Deserialize)]
pub struct SearchResponse {
    #[serde(rename = "startAt")]
    pub start_at: i64,
    #[serde(rename = "maxResult")]
    pub max_results: i64,
    pub total: i64,
    pub issues: Vec<Issue>,
}