pub struct Parser {
}

impl Parser {
    pub fn parse_github_owner_and_repo(input: &str) -> Option<Vec<(String, String)>> {
        let mut result = vec![];
        let re = regex::Regex::new(r"https://github.com/([\w\s\-_]+)/([\w\s\-_]+)").ok()?;
        for cap in re.captures_iter(&input) {
            let caps = (cap.get(1), cap.get(2));
            match caps {
                (Some(owner), Some(repo)) => {
                    result.push((owner.as_str().to_string(), repo.as_str().to_string()))
                },
                (_, _) => {
                    return None;
                }
            }
        }
        Some(result)
    }
}

#[test]
fn test_parse_str() {
    let input = r##"
  * [carllerche/tower-web](https://github.com/carllerche/tower-web) [[tower-web](https://crates.io/crates/tower-web)] — A fast, boilerplate free, web framework for Rust [<img src="https://api.travis-ci.org/carllerche/tower-web.svg?branch=master">](https://travis-ci.org/carllerche/tower-web)
  * [danclive/sincere](https://github.com/danclive/sincere) — A micro web framework for Rust(stable) based on hyper and multithreading. [<img src="https://api.travis-ci.org/danclive/sincere.svg?branch=master">](https://travis-ci.org/danclive/sincere)
  * [oltdaniel/zap](https://github.com/oltdaniel/zap) — A lightning fast http framework for Rust [<img src="https://api.travis-ci.org/oltdaniel/zap.svg?branch=master">](https://travis-ci.org/oltdaniel/zap)
  * [actix/sockjs](https://github.com/actix/sockjs) — A [SockJS](https://github.com/sockjs) server for Rust [<img src="https://api.travis-ci.org/actix/sockjs.svg?branch=master">](https://travis-ci.org/actix/sockjs)
  * [cyderize/rust-websocket](https://github.com/cyderize/rust-websocket) — A framework for dealing with WebSocket connections (both clients and servers) [<img src="https://api.travis-ci.org/cyderize/rust-websocket.svg?branch=master">](https://travis-ci.org/cyderize/rust-websocket)
  * [housleyjk/ws-rs](https://github.com/housleyjk/ws-rs) — lightweight, event-driven WebSockets for Rust [<img src="https://api.travis-ci.org/housleyjk/ws-rs.svg?branch=stable">](https://travis-ci.org/housleyjk/ws-rs)
"##;
    let result = Parser::parse_github_owner_and_repo(input);
    assert!(result.is_some());
    let result = result.unwrap();
    assert_eq!(result.len(), 6);
    println!("{:?}", result);
}