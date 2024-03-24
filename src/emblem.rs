
use engage::menu::{config::{ConfigBasicMenuItemSwitchMethods, ConfigBasicMenuItem}, BasicMenuResult};
use unity::prelude::*;

use crate::config::CONFIG;

pub struct EmblemLeaveSetting;

impl ConfigBasicMenuItemSwitchMethods for EmblemLeaveSetting { 
    fn init_content(_this: &mut ConfigBasicMenuItem) {
        let _toggle = CONFIG.lock().unwrap().godescape;
    }

    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        let result = ConfigBasicMenuItem::change_key_value_b(CONFIG.lock().unwrap().godescape);
        if CONFIG.lock().unwrap().godescape != result {
            CONFIG.lock().unwrap().godescape = result;
            Self::set_help_text(this, None);
            Self::set_command_text(this, None);
            this.update_text();
            // Update the config here by writing if the value changed.
            CONFIG.lock().unwrap().write();
            BasicMenuResult::se_cursor()
        } else {
            BasicMenuResult::new()
        }
    }

    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) {
        if CONFIG.lock().unwrap().godescape {
            this.command_text = "On".into();
        } else {
            this.command_text = "Off".into();
        }
    }

    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) {
        if CONFIG.lock().unwrap().godescape {
            this.help_text = "Emblems will not be lost.".into();
        } else {
            this.help_text = "Emblems will be lost.".into();
        }
    }
}

/* #[no_mangle]
extern "C" fn emblemleave_callback() -> &'static mut ConfigBasicMenuItem {
    ConfigBasicMenuItem::new_switch::<EmblemLeaveSetting>("Toggle GodEscape")
}


pub fn emblemleave_install() {
    cobapi::install_global_game_setting(emblemleave_callback);
} */