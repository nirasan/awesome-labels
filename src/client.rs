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
            let issue: Issue = serde_json::from_value(issue.to_owned()).unwrap();
            result.push(issue);
        }
        return Some(result);
    }

    fn get_json (
        response: Result<(HeaderMap, StatusCode, Option<Value>), github_rs::errors::Error>,
    ) -> Option<Value> {
        match response {
            Ok((headers, status, json)) => {
                if status.is_success() {
                    json
                } else {
                    None
                }
            }
            Err(e) => {
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

#[derive(Serialize, Deserialize, Debug)]
struct Issue {
    labels: Vec<Label>
}

#[derive(Serialize, Deserialize, Debug)]
struct Label {
    color: String,
    name: String,
}
