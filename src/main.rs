#![deny(warnings)]
extern crate hyper;
extern crate pretty_env_logger;

use hyper::{Body, Request, Response, Server, StatusCode};
use hyper::service::service_fn_ok;
use hyper::rt::{self, Future};
use std::env;
use awesome_labels::runner::Runner;
use awesome_labels::client::Client;
use awesome_labels::structs::*;
use url::form_urlencoded::byte_serialize;

fn main() {
    pretty_env_logger::init();

    let mut port: u16 = 8080;
    match env::var("PORT") {
        Ok(p) => {
            match p.parse::<u16>() {
                Ok(n) => {port = n;},
                Err(_e) => {},
            };
        }
        Err(_e) => {},
    };
    let addr = ([0, 0, 0, 0], port).into();

    let new_service = || {
        service_fn_ok(update_target)
    };

    let server = Server::bind(&addr)
        .serve(new_service)
        .map_err(|e| eprintln!("server error: {}", e));

    println!("Listening on http://{}", addr);

    rt::run(server);
}

fn update_target(_req: Request<Body>) -> Response<Body> {
    let token = match env::var("GITHUB_TOKEN") {
        Ok(val) => val,
        Err(e) => {
            eprintln!("{}", e);
            return Response::builder().status(StatusCode::BAD_REQUEST).body(Body::from("")).unwrap();
        },
    };

    let url = "https://raw.githubusercontent.com/nirasan/awesome-labels/master/docs/test.md";

    let runner = Runner::new(token.as_str());
    let labels = runner.run(url);

    let mut file_content = String::new();
    file_content.push_str("|label name|issues count|repos count|url|\n");
    file_content.push_str("|---|---|---|---|\n");
    for label in labels {
        let query: String = byte_serialize(format!(r#"is:issue is:open label:"{}""#, label.name).as_bytes()).collect();
        let url = format!("https://github.com/search?q={}", query);
        file_content.push_str(&format!("|{}|{}|{}|{}|\n", label.name, label.issues_count, label.repos_count, url));
    }

    let client = Client::new(token.as_str());
    let content = client.get_contents("nirasan", "awesome-labels", "docs/target.md").unwrap();
    let message = format!("test message. sha is {}", &content.sha);
    let payload = GithubContentPayload {
        sha: content.sha,
        message: message,
        content: base64::encode(&file_content),
    };
    let contents = client.put_contents("nirasan", "awesome-labels", "docs/test.md", payload);
    if contents.is_none() {
        return Response::builder().status(StatusCode::INTERNAL_SERVER_ERROR).body(Body::from("")).unwrap();
    }

    return Response::builder().status(StatusCode::OK).body(Body::from("")).unwrap();
}