extern crate rust_web_client_tutorial as client;
fn main() {
    match client::run() {
        Err(e) => println!("Error: {}", e),
        Ok(()) => (),
    }
}
