extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::env;
use std::fs::File;
use std::io::Read;

mod model;

fn main() {
    let path = env::home_dir().map(|x| x.join(".tokens/github")).unwrap();
    if path.exists() {
        let mut file = File::open(path).expect("Unable to open token file");
        let mut token = String::new();
        file.read_to_string(&mut token)
            .expect("Unable to read token");
        println!("{}", token.trim());

        let uri = format!(
            "https://api.github.com/repos/rust-lang/rust/pulls?access_token={}",
            token.trim()
        );

        let body: Vec<model::PullRequest> = reqwest::get(&uri).unwrap().json().unwrap();

        for elem in body {
            println!("{} [{}] - {}", elem.number, elem.state, elem.title);
            println!()
        }
    }
}
