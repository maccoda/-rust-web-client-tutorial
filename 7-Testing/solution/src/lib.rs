extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::fs::File;
use std::io::Read;
use std::path::Path;

pub mod error;
mod model;

pub struct RustClient {
    token: String,
}

impl RustClient {
    pub fn new(token: String) -> RustClient {
        RustClient { token }
    }

    pub fn get_pull_requests(&self) -> Result<Vec<model::PullRequest>, error::Error> {
        let uri = format!(
            "https://api.github.com/repos/rust-lang/rust/pulls?access_token={}",
            self.token.trim()
        );

        let body: Vec<model::PullRequest> = reqwest::get(&uri)?.json()?;
        Ok(body)
    }
}

pub fn obtain_token(path: &Path) -> Result<String, error::Error> {
    if path.exists() {
        let mut file = File::open(path)?;
        let mut token = String::new();
        file.read_to_string(&mut token)?;
        Ok(token)
    } else {
        Err(error::Error::TokenNotFound)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn should_return_error_when_no_token_file() {
        // Need to import as the imports from parent module are not passed down
        use std::path::Path;
        // Access the parent module through the super keyword
        let result = super::obtain_token(Path::new("this_path/definitely/does_not/exist"));

        // Macro to assert the provided arguments are true
        assert!(result.is_err());
    }

    #[test]
    fn should_return_contents_of_token_file() {
        use std::fs::{self, File};
        use std::io::Write;
        use std::path::Path;

        let path = Path::new("my_token");

        // In tests we usually just unwrap as we want the test to fail early
        let mut file = File::create(&path).unwrap();
        file.write_all(b"token").unwrap();

        let result = super::obtain_token(&path);

        assert!(result.is_ok());
        // Another macro for testing to check both arguments are equal. To use
        // this they both must implement std::cmp::PartialEq
        assert_eq!("token", result.unwrap());
        fs::remove_file(&path).unwrap();
    }

    #[test]
    fn should_store_token_in_client() {
        let client = super::RustClient::new("token".to_string());

        // Since this is in a sub module it has access to the paramter
        assert_eq!("token", client.token);
    }
}
