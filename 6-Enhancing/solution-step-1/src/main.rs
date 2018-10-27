extern crate rust_web_client_tutorial as client;

use std::env;

fn main() {
    match run() {
        Err(e) => println!("Error: {}", e),
        Ok(()) => (),
    }
}

fn run() -> Result<(), client::error::Error> {
    let path = env::home_dir().map(|x| x.join(".tokens/github")).unwrap();
    let token = client::obtain_token(&path)?;
    let rs_client = client::RustClient::new(token);
    let body = rs_client.get_pull_requests()?;
    for elem in body {
        println!("{} [{}] - {}", elem.number, elem.state, elem.title);
        println!()
    }
    Ok(())
}
