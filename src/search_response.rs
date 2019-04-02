use issue::Issue;

#[derive(Deserialize, Serialize)]
pub struct SearchResponse {
    #[serde(rename = "startAt")]
    pub start_at: i64,
    #[serde(rename = "maxResults")]
    pub max_results: i64,
    pub total: i64,
    pub issues: Vec<Issue>,
}
