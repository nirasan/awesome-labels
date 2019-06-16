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
    fn get_issues(&self, owner: &str, repo_name: &str) -> Option<Vec<Value>> {
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
        Some(issues.to_owned())
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
    let res = client.get_issues("rust-lang", "rfcs");
    assert_ne!(res, None);
    let res = res.unwrap();
    assert!(res.len() > 0);
    println!("{:?}", res);
}
