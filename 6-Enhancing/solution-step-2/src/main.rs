extern crate dirs;
extern crate rust_web_client_tutorial as client;
fn main() {
    let path = dirs::home_dir().map(|x| x.join(".tokens/github")).unwrap();
    match client::obtain_token(&path) {
        Err(e) => println!("Error: {}", e),
        Ok(token) => {
            let rs_client = client::RustClient::new(token);
            match rs_client.get_pull_requests() {
                Err(e) => println!("Error: {}", e),
                Ok(body) => {
                    for elem in body {
                        println!("{} [{}] - {}", elem.number, elem.state, elem.title);
                        println!()
                    }
                }
            }
        }
    }
}
