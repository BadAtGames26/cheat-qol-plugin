#![feature(lazy_cell, ptr_sub_ptr)]

use unity::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{LazyLock, Mutex};

mod emblem;
mod arena;
mod gameparam;
mod event;
mod rewind;
mod summon;
mod ring;
mod silvercard;

// The path for the config file
pub const CONFIG_PATH: &str = "sd:/engage/config/badconfig.toml";
pub static CONFIG: LazyLock<Mutex<Config>> = LazyLock::new(|| Config::new().into());

#[derive(Default, Deserialize, Serialize)]
pub struct Config {
    godescape: bool,
    arenalimit: i32,
    rewind: i32,
    summon: i32,
    ring: i32,
    discount: f32,
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
                // This is mostly intended to create a new file if more items are added to the struct
                println!("Config file could not be parsed, a default config file has been created.");
                let config = Config::default();
                config
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
        };
        config
    }
}

#[unity::hook{"App","ScriptUnit","GodUnitSetEscape"}]
pub fn godunitsetescape(args: &u64, method_info: OptionalMethod) {
    // Check what the config bool is and do not call the function if it is true.
    if CONFIG.lock().unwrap().godescape {
        println!("GodEsacpe was skipped due to settings.");
    } else {
        // Call the original function if the bool is false, default behavior.
        call_original!(args, method_info);
    }
}

#[skyline::main(name = "cheat-qol")]
pub fn main() {
    std::panic::set_hook(Box::new(|info| {
        let location = info.location().unwrap();

        let msg = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => {
                match info.payload().downcast_ref::<String>() {
                    Some(s) => &s[..],
                    None => "Box<Any>",
                }
            },
        };


        let err_msg = format!(
            "Cheat/QOL plugin has panicked at '{}' with the following message:\n{}\0",
            location,
            msg
        );

        skyline::error::show_error(
            420,
            "Cheat/QOL plugin has panicked! Please open the details and send a screenshot to the developer, then close the game.\n\0",
            err_msg.as_str(),
        );
    }));
    LazyLock::force(&CONFIG) ;
    emblem::emblemleave_install();
    arena::arenalimit_install();
    rewind::rewind_install();
    summon::summon_install();
    ring::ring_install();
    event::listener_install();
    silvercard::discount_install();
    skyline::install_hooks!(godunitsetescape);
}
