use issue::Issue;

#[derive(Deserialize)]
pub struct SearchResponse {
    pub expand: String,
    pub startAt: i64,
    pub maxResult: i64,
    pub issues: Vec<Issue>,
}