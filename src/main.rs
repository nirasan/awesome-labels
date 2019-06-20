use github_rs::client::{Executor, Github};
use github_rs::{HeaderMap, StatusCode};
use hyper::header::{HeaderValue, ACCEPT};
use serde_json::Value;
use std::fs;

fn main() {

}

#[test]
fn test_regex_github() {
    let content = fs::read_to_string("./secret.txt").ok().unwrap();
    let client = Github::new(&content).expect("failed to create client");

    let mut map = std::collections::HashMap::new();

    let content = fs::read_to_string("./src/awesome-rust-README.md").ok().unwrap();

    let re = regex::Regex::new(r"https://github.com/([\w\s\-_]+)/([\w\s\-_]+)").unwrap();
    for cap in re.captures_iter(&content) {
        println!("owner: {}, repo_name: {}\n", &cap[1], &cap[2]);

        let res = client.get().repos().owner(&cap[1]).repo(&cap[2]).issues().execute::<Value>();
        let issues = print_info_and_get_json(res);
        if let None = issues {
            continue;
        }
        let issues = issues.unwrap();
        let issues = issues.as_array();
        if let None = issues {
            continue;
        }
        let issues = issues.unwrap();
        for issue in issues {
            let labels = &issue["labels"];
            let labels = labels.as_array().unwrap();
            for label in labels {
                let name = label["name"].as_str().unwrap();
                let count = map.get(name);
                let count = match count {
                    Some(c) => c + 1,
                    None => 1,
                };
                map.insert(name.to_owned(), count);
            }
        }
        break;
    }

    println!("[map] {:?}", map);
}

#[test]
fn test_regex() {
    let content = fs::read_to_string("./src/awesome-rust-README.md").ok().unwrap();

    let re = regex::Regex::new(r"https://github.com/([\w\s\-_]+)/([\w\s\-_]+)").unwrap();
    for cap in re.captures_iter(&content) {
        println!("owner: {}, repo_name: {}\n", &cap[1], &cap[2]);
    }
}

fn get_issues_by_owner_and_repo(owner: &str, repo_name: &str) {
    let content = fs::read_to_string("./secret.txt").ok().unwrap();

    //create new client
    let client = Github::new(&content).expect("failed to create client");
//    let labels = get_labels(&client, owner, repo_name).expect("failed to get labels");
//    println!("{:?}", labels);
//
//    let issues = get_issues(&client, owner, repo_name).expect("failed to get labels");
//    println!("{:?}", issues);

    let res = client.get().repos().owner(owner).repo(repo_name).issues().execute::<Value>();
    let issues = print_info_and_get_json(res).unwrap();
    println!("[issues] {:?}", issues);
    let issues = issues.as_array().unwrap();
    for issue in issues {
        println!("[labels] {:?}", issue["labels"]);
        let labels = &issue["labels"];
        let labels = labels.as_array().unwrap();
        for label in labels {
            println!("[label] {:?}", label);
            let name = label["name"].as_str().unwrap();
            println!("[name] {:?}", name);
        }
    }
}

#[test]
fn test_get_issues_by_owner_and_repo() {
    get_issues_by_owner_and_repo("rust-lang", "rfcs");
}

//get all issues for repo
fn get_issues(client: &Github, owner: &str, repo_name: &str) -> Option<Value> {
    //endpoint found on https://developer.github.com/v3/issues/#list-issues-for-a-repository
    let issues_endpoint = format!("repos/{}/{}/issues", owner, repo_name);
    //execute
    let response = client
        .get()
        //set custom endpoint here
        .custom_endpoint(&issues_endpoint)
        .execute::<Value>();
    print_info_and_get_json(response)
}

fn get_labels(client: &Github, owner: &str, repo_name: &str) -> Option<Value> {
    //endpoint found on https://developer.github.com/v3/issues/#list-issues-for-a-repository
    let issues_endpoint = format!("repos/{}/{}/labels", owner, repo_name);
    //execute
    let response = client
        .get()
        //set custom endpoint here
        .custom_endpoint(&issues_endpoint)
        .execute::<Value>();
    print_info_and_get_json(response)
}

//get reactions for particular issue
fn get_reactions(
    client: &Github,
    owner: &str,
    repo_name: &str,
    issue_number: u64,
) -> Option<Value> {
    //build endpoint
    let reactions_endpoint = format!(
        "repos/{}/{}/issues/{}/reactions",
        owner, repo_name, issue_number
    );

    println!("reactions endpoint: {:?}", &reactions_endpoint);
    //send request with custom header
    let reactions_response = client
        .get()
        .custom_endpoint(&reactions_endpoint)
        .set_header(
            ACCEPT,
            HeaderValue::from_static("application/vnd.github.squirrel-girl-preview"),
        )
        .execute::<Value>();

    print_info_and_get_json(reactions_response)
}

//printing headers and status or error and returning json on success
fn print_info_and_get_json(
    response: Result<(HeaderMap, StatusCode, Option<Value>), github_rs::errors::Error>,
) -> Option<Value> {
    match response {
        Ok((headers, status, json)) => {
            println!("{:#?}", headers);
            println!("{}", status);
            json
        }
        Err(e) => {
            println!("{}", e);
            None
        }
    }
}
