#[derive(Serialize, Deserialize, Debug)]
pub struct GithubIssue {
    pub labels: Vec<GithubLabel>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GithubLabel {
    pub color: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GithubContent {
    pub sha: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GithubContentPayload {
    pub message: String,
    pub content: String,
    pub sha: String,
}