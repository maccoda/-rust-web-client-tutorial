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
    }
}
