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

#[derive(Serialize, Deserialize, Debug)]
pub struct Label {
    pub name: String,
    pub issues_count: i32,
    pub repos_count: i32,
    pub url: String,
}
