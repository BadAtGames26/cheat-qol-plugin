#![feature(lazy_cell, ptr_sub_ptr)]

use unity::prelude::*;

mod config;
mod emblem;
mod arena;
mod gameparam;
mod event;
mod rewind;
mod summon;
mod ring;
mod silvercard;
mod well;

use crate::config::CONFIG;
use std::sync::LazyLock;

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
    well::well_install();
    skyline::install_hooks!(godunitsetescape);
}
