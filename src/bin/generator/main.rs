extern crate awesome_labels;
extern crate reqwest;

use awesome_labels::structs::*;
use awesome_labels::client::Client;
use awesome_labels::parser::Parser;
use std::fs;
use std::io::{BufWriter, Write};
use std::collections::HashMap;

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

    let mut total_count_table: HashMap<String, i32> = HashMap::new();
    let mut repo_count_table: HashMap<String, i32> = HashMap::new();

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
                let count = total_count_table.get(name);
                let count = match count {
                    Some(n) => n + 1,
                    None => 1,
                };
                total_count_table.insert(name.to_owned(), count);
            }
        }

        for (name, _) in &repo_table {
            let count = repo_count_table.get(name);
            let count = match count {
                Some(n) => n + 1,
                None => 1,
            };
            repo_count_table.insert(name.to_owned(), count);
        }
    }

    let mut f = BufWriter::new(fs::File::create(file).unwrap());

    writeln!(f, "[total count] {:?}", total_count_table).unwrap();
    writeln!(f, "[repo count] {:?}", repo_count_table).unwrap();

    f.flush().expect("failed to flush");
}
