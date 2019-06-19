use github_rs::client::{Executor, Github};
use github_rs::{HeaderMap, StatusCode};
use serde_json::Value;

struct Client {
    github: Github
}

impl Client {
    fn new(token: &str) -> Client {
        let github = Github::new(token).ok().expect("failed to create github client");
        Client{
            github
        }
    }

    //get all issues for repo
    fn get_issues(&self, owner: &str, repo_name: &str) -> Option<Vec<Issue>> {
        //endpoint found on https://developer.github.com/v3/issues/#list-issues-for-a-repository
        let issues_endpoint = format!("repos/{}/{}/issues", owner, repo_name);
        //execute
        let response = self
            .github
            .get()
            //set custom endpoint here
            .custom_endpoint(&issues_endpoint)
            .execute::<Value>();
        let json = Self::get_json(response)?;
        let issues = json.as_array()?;
        let mut result = vec![];
        for issue in issues {
            let issue: Issue = serde_json::from_value(issue.to_owned()).ok()?;
            result.push(issue);
        }
        return Some(result);
    }

    fn get_contents(&self, owner: &str, repo_name: &str, path: &str) -> Option<Content> {
        let endpoint = format!("repos/{}/{}/contents/{}", owner, repo_name, path);
        let response = self
            .github
            .get()
            .custom_endpoint(&endpoint)
            .execute::<Value>();
        let json= Self::get_json(response)?;
        let content: Content = serde_json::from_value(json).ok()?;
        return Some(content);
    }

    fn put_contents(&self, owner: &str, repo_name: &str, path: &str, payload: ContentPayload) -> Option<()> {
        let endpoint = format!("repos/{}/{}/contents/{}", owner, repo_name, path);
        let response = self
            .github
            .put(payload)
            .custom_endpoint(&endpoint)
            .execute::<Value>();
        println!("[response] {:?}", response);
        let json= Self::get_json(response)?;
        println!("{}", serde_json::to_string_pretty(&json).unwrap());
        return Some(());
    }

    fn get_json (
        response: Result<(HeaderMap, StatusCode, Option<Value>), github_rs::errors::Error>,
    ) -> Option<Value> {
        match response {
            Ok((_, status, json)) => {
                if status.is_success() {
                    json
                } else {
                    None
                }
            }
            Err(_) => {
                None
            }
        }
    }
}

#[test]
fn test_get_issues() {
    let content = std::fs::read_to_string("./secret.txt").ok().unwrap();
    let client = Client::new(&content);
    let issues = client.get_issues("rust-lang", "rfcs");
    assert!(issues.is_some());
    let issues = issues.unwrap();
    assert!(issues.len() > 0);
    for issue in issues {
        println!("{:?}", issue);
    }
}

#[test]
fn test_get_contents() {
    let content = std::fs::read_to_string("./secret.txt").ok().unwrap();
    let client = Client::new(&content);
    let contents = client.get_contents("nirasan", "awesome-labels", "docs/test.md");
    assert!(contents.is_some());
    let contents = contents.unwrap();
    // println!("{}", serde_json::to_string_pretty(&contents).unwrap());
    println!("{:?}", contents);
}

#[test]
fn test_put_contents() {
    let content = std::fs::read_to_string("./secret.txt").ok().unwrap();
    let client = Client::new(&content);
    let content = client.get_contents("nirasan", "awesome-labels", "docs/test.md").unwrap();
    let file_content = r##"# header

## header

* list
  * sub list
  * sub list
* list
  * new sub list
  * test sub list"##;
    let message = format!("test message. sha is {}", &content.sha);
    let payload = ContentPayload{
        sha: content.sha,
        message: message,
        content: base64::encode(file_content),
    };
    let contents = client.put_contents("nirasan", "awesome-labels", "docs/test.md", payload);
    assert!(contents.is_some());
}

#[derive(Serialize, Deserialize, Debug)]
struct Issue {
    labels: Vec<Label>
}

#[derive(Serialize, Deserialize, Debug)]
struct Label {
    color: String,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Content {
    sha: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ContentPayload {
    message: String,
    content: String,
    sha: String,
}