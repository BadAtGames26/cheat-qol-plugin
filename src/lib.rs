#![feature(ptr_sub_ptr)]

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
mod settingsmenu;

use crate::config::QOLCONFIG;
use std::sync::LazyLock;
use skyline::hooks::InlineCtx;

// Using an inline hook here is safer than just skipping GodEscape entirely since this will make calls that set IsEscaping to false still run
#[skyline::hook(offset=0x021a0b6c, inline)]
pub fn godescape_hook(ctx: &mut InlineCtx) {
    if QOLCONFIG.lock().unwrap().godescape {
        ctx.registers[8].set_x(0);
        println!("GodEscape was set to false.")
    }
}

#[skyline::main(name = "cheatqol")]
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
    LazyLock::force(&QOLCONFIG) ;
    event::listener_install();
    settingsmenu::submenu_install();
    skyline::install_hooks!(godescape_hook);
}
