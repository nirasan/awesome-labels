extern crate awesome_labels;
extern crate reqwest;

use awesome_labels::client::Client;
use awesome_labels::parser::Parser;
use std::fs;
use std::io::{BufWriter, Write};
use std::collections::HashMap;
use url::form_urlencoded::byte_serialize;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 4 {
        panic!("invalid arguments. {:?}", args);
    }

    let url = &args[1];
    let token = &args[2];
    let file = &args[3];

    let body = reqwest::get(url).expect("failed to get request").text().expect("failed to get text");
    let map = Parser::parse_github_owner_and_repo(&body).expect("failed to parse owner and repo");

    let client = Client::new(token);

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

    let mut f = BufWriter::new(fs::File::create(file).unwrap());
    
    writeln!(f, "|label name|issues count|repos count|url|").unwrap();
    writeln!(f, "|---|---|---|---|").unwrap();

    for pair in count_vec {
        let name = pair.0;
        let issues = *issues_counter.get(name).unwrap_or(&0);
        let repos = *repos_counter.get(name).unwrap_or(&0);

        let query: String = byte_serialize(format!(r#"is:issue is:open label:"{}""#, name).as_bytes()).collect();
        let url = format!("https://github.com/search?q={}", query);

        writeln!(f, "|{}|{}|{}|{}|", name, issues, repos, url).unwrap();
    }

    f.flush().expect("failed to flush");
}
