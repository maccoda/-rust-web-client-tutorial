extern crate hyper;
extern crate hyper_tls;
use hyper_tls::HttpsConnector;

use hyper::client::HttpConnector;
use hyper::rt::{self, Future, Stream};
use hyper::{Body, Client, Request};
use std::io::{self, Write};

use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let path = env::home_dir().map(|x| x.join(".tokens/github")).unwrap();
    if path.exists() {
        let mut file = File::open(path).expect("Unable to open token file");
        let mut token = String::new();
        file.read_to_string(&mut token)
            .expect("Unable to read token");
        println!("{}", token.trim());

        rt::run(client(token.trim()))
    }
}

fn client(token: &str) -> impl Future<Item = (), Error = ()> {
    let client = build_client();
    let uri = format!(
        "https://api.github.com/repos/rust-lang/rust/pulls?access_token={}",
        token
    );

    let request = Request::get(uri)
        .header("User-Agent", "my-awesome-agent/1.0")
        .body(Body::empty())
        .unwrap();

    client
        .request(request)
        .and_then(|res| {
            println!("Response: {}", res.status());
            res
            .into_body()
            // Body is a stream, so as each chunk arrives...
            .for_each(|chunk| {
                io::stdout()
                    .write_all(&chunk)
                    .map_err(|e| {
                        panic!("example expects stdout is open, error={}", e)
                    })
            })
        }).map_err(|err| {
            println!("Error: {}", err);
        })
}

fn build_client() -> Client<HttpsConnector<HttpConnector>> {
    let https = HttpsConnector::new(4).expect("TLS initialization failed");
    Client::builder().build(https)
}
