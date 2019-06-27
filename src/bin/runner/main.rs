use std::fs;
use std::io::{BufWriter, Write};
use awesome_labels::runner::Runner;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 4 {
        panic!("invalid arguments. {:?}", args);
    }

    let url = &args[1];
    let token = &args[2];
    let file = &args[3];

    let runner = Runner::new(token);
    let labels = runner.run(url);

    let mut f = BufWriter::new(fs::File::create(file).unwrap());
    let json = serde_json::to_string(&labels).expect("failed to encoding json");
    writeln!(f, "{}", json).unwrap();

    /*
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
    */

    f.flush().expect("failed to flush");
}
