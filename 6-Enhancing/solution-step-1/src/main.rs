extern crate rust_web_client_tutorial as client;
fn main() {
    match client::obtain_token() {
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
