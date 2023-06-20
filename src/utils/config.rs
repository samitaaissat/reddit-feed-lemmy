use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub instance: String,
    pub user: String,
    pub pass: String,
    pub subreddits: Vec<String>,
}

impl Config {
    // The load method returns a Config struct.
    pub fn load() -> Config {
        // Read the config.json file.
        let path = Path::new("config.json");
        let mut file = match File::open(&path) {
            Err(why) => panic!("Couldn't open {}: {}", path.display(), why),
            Ok(file) => file,
        };

        // Parse the config.json file using serde
        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Err(why) => panic!("Couldn't read {}: {}", path.display(), why),
            Ok(_) => (),
        }

        // Return the Config struct.
        match serde_json::from_str(&contents) {
            Err(why) => panic!("Couldn't parse {}: {}", path.display(), why),
            Ok(config) => config,
        }
    }
}
