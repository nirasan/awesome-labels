use crate::client::Client;
use crate::parser::Parser;
use crate::structs::*;
use std::collections::HashMap;
use url::form_urlencoded::byte_serialize;
use hyper_tls::HttpsConnector;
use hyper::Client;

pub struct Runner {
    token: String,
}

impl Runner {
    pub fn new(token: &str) -> Runner {
        Runner{
            token: token.to_string(),
        }
    }

    pub fn run(&self, url: &str) -> Vec<Label> {
        let https = HttpsConnector::new(4).unwrap();
        let hyper_client = Client::builder()
            .build::<_, hyper::Body>(https);


        let body = reqwest::get(url).expect("failed to get request").text().expect("failed to get text");
        let map = Parser::parse_github_owner_and_repo(&body).expect("failed to parse owner and repo");

        let client = Client::new(&self.token);

        let mut issues_counter: HashMap<String, i32> = HashMap::new();
        let mut repos_counter: HashMap<String, i32> = HashMap::new();

        for (owner, repo) in &map {
            let issues = client.get_issues(owner, repo);
            if issues.is_none() {
                continue;
            }
            let issues = issues.unwrap();

            let mut repo_table: HashMap<String, bool> = HashMap::new();
            for issue in issues {
                for label in issue.labels {
                    let name = &label.name;
                    repo_table.insert(name.to_owned(), true);
                    let count = *issues_counter.get(name).unwrap_or(&0) + 1;
                    issues_counter.insert(name.to_owned(), count);
                }
            }

            for (name, _) in &repo_table {
                let count = *repos_counter.get(name).unwrap_or(&0) + 1;
                repos_counter.insert(name.to_owned(), count);
            }
        }

        let mut count_vec: Vec<_> = issues_counter.iter().collect();
        count_vec.sort_by(|a, b| b.1.cmp(a.1));

        let mut labels = vec![];
        for pair in count_vec {
            let name = pair.0.to_owned();

            let issues_count = *issues_counter.get(&name).unwrap_or(&0);
            let repos_count = *repos_counter.get(&name).unwrap_or(&0);

            let query: String = byte_serialize(format!(r#"is:issue is:open label:"{}""#, name).as_bytes()).collect();
            let url = format!("https://github.com/search?q={}", query);

            labels.push(Label{
                name,
                issues_count,
                repos_count,
                url,
            });
        }
        return labels;
    }
}

#[test]
fn test_run() {
    let token = std::fs::read_to_string("./secret.txt").ok().unwrap();
    let runner = Runner::new(token);
    let labels = runner.run("https://raw.githubusercontent.com/nirasan/awesome-labels/master/docs/test.md");
    assert!(labels.len() > 0);
    println!("{:?}", labels);
}