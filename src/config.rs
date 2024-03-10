use serde::{Deserialize, Serialize};
use std::sync::{LazyLock, Mutex};


// The path for the config file
pub const CONFIG_PATH: &str = "sd:/engage/config/badconfig.toml";
pub static CONFIG: LazyLock<Mutex<Config>> = LazyLock::new(|| Config::new().into());

#[derive(Default, Deserialize, Serialize)]
pub struct Config {
    pub godescape: bool,
    pub arenalimit: i32,
    pub rewind: i32,
    pub summon: i32,
    pub ring: i32,
    pub discount: f32,
    pub well: i32,

}

impl Config {
    pub fn new() -> Self {
        let config_content = std::fs::read_to_string(CONFIG_PATH);
        // If the file is read to a string or there is no failure, parse into the config struct.
        if config_content.is_ok() {
            let content = config_content.unwrap();
            let config = toml::from_str(&content);
            if config.is_ok() {
                println!("Config file was parsed with no issues.");
                let config = config.unwrap();
                config
            } else {
                panic!("Config file has failed to parse.");
            }
        } else {
            // If the file could not be read to a string then create a new file with default values.
            println!("The config file was either missing or unable to be read, creating new toml.");
            let config = Config::default();
            config.write();
            config
        }
    }

    // Function to write to a file, makes it look nice then writes to the path.
    pub fn write(&self) {
        let content = toml::to_string_pretty(&self).expect("Could not create toml from config.");
        std::fs::write(CONFIG_PATH, &content).expect("Could not write new toml to sd:/engage/config");
    }

    pub fn default() -> Self {
        let config = Config {
            godescape: false,
            arenalimit: 0,
            rewind: 0,
            summon: 0,
            ring: 0,
            discount: 0.3,
            well: 0,
        };
        config
    }
}