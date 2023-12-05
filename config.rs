use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub database_url: String,
    pub mft_file_path: PathBuf,
}

impl Config {
    pub fn new() -> anyhow::Result<Self> {
        // Here you can implement the logic to load the configuration, for example from a file or environment variables.
        // For simplicity, we are hardcoding the values. In a real-world scenario, you would want to avoid this.

        // Example hardcoded configuration:
        let database_url = "sqlite:mft_data.db".to_string(); // This should be replaced with actual configuration retrieval logic
        let mft_file_path = PathBuf::from("C:\\path\\to\\MFT"); // This should be replaced with actual configuration retrieval logic

        Ok(Config {
            database_url,
            mft_file_path,
        })
    }
}
