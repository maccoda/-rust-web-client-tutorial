#[derive(Deserialize)]
pub struct PullRequest {
    pub number: usize,
    pub state: String,
    pub title: String,
}
