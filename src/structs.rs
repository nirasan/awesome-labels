#[derive(Serialize, Deserialize, Debug)]
pub struct Issue {
    pub labels: Vec<Label>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Label {
    pub color: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Content {
    pub sha: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContentPayload {
    pub message: String,
    pub content: String,
    pub sha: String,
}