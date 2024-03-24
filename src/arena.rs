use engage::menu::{config::{ConfigBasicMenuItem, ConfigBasicMenuItemSwitchMethods}, BasicMenuResult};
use unity::prelude::*;

use crate::{config::CONFIG, event::*};

pub struct ArenaLimitSetting;

impl ConfigBasicMenuItemSwitchMethods for ArenaLimitSetting { 
    fn init_content(_this: &mut ConfigBasicMenuItem) {
        let _value = CONFIG.lock().unwrap().arenalimit;
    }

    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        let result = ConfigBasicMenuItem::change_key_value_i(CONFIG.lock().unwrap().arenalimit, 0, 5, 1);
        if CONFIG.lock().unwrap().arenalimit != result {
            CONFIG.lock().unwrap().arenalimit = result;
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
        this.command_text = match CONFIG.lock().unwrap().arenalimit {
            1 => "0".into(),
            2 => "5".into(),
            3 => "10".into(),
            4 => "100".into(),
            5 => "1000".into(),
            _ => "Default".into(),
        }
    }

    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) {
        this.help_text = "The amount of battles that can be performed in the Arena.".into();
    }
}

/* #[no_mangle]
extern "C" fn arenalimit_callback() -> &'static mut ConfigBasicMenuItem {
    ConfigBasicMenuItem::new_switch::<ArenaLimitSetting>("Arena Limit Value")
}


pub fn arenalimit_install() {
    cobapi::install_global_game_setting(arenalimit_callback);
} */