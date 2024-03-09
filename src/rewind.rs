use engage::menu::{config::{ConfigBasicMenuItem, ConfigBasicMenuItemSwitchMethods}, BasicMenuResult};
use unity::prelude::*;

use crate::{config::CONFIG, event::*};

pub struct RewindSetting;

impl ConfigBasicMenuItemSwitchMethods for RewindSetting { 
    fn init_content(_this: &mut ConfigBasicMenuItem) {
        let _value = CONFIG.lock().unwrap().rewind;
    }

    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        let result = ConfigBasicMenuItem::change_key_value_i(CONFIG.lock().unwrap().rewind, 0, 3, 1);
        if CONFIG.lock().unwrap().rewind != result {
            CONFIG.lock().unwrap().rewind = result;
            Self::set_help_text(this, None);
            Self::set_command_text(this, None);
            this.update_text();
            rewind_change();
            // Update the config here by writing if the value changed.
            CONFIG.lock().unwrap().write();
            BasicMenuResult::se_cursor()
        } else {
            BasicMenuResult::new()
        }
    }

    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) {
        this.command_text = match CONFIG.lock().unwrap().rewind {
            1 => "0".into(),
            2 => "10".into(),
            3 => "Unlimited".into(),
            _ => "Default".into(),
        }
    }

    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) {
        this.help_text = "The number of rewinds that can be used in a battle.".into();
    }
}

#[no_mangle]
extern "C" fn rewind_callback() -> &'static mut ConfigBasicMenuItem {
    ConfigBasicMenuItem::new_switch::<RewindSetting>("Rewind Value")
}


pub fn rewind_install() {
    cobapi::install_global_game_setting(rewind_callback);
}


