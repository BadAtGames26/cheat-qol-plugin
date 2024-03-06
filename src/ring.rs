use engage::menu::{config::{ConfigBasicMenuItem, ConfigBasicMenuItemSwitchMethods}, BasicMenuResult};
use unity::prelude::*;

use crate::{CONFIG, event::*};

pub struct RingSetting;

impl ConfigBasicMenuItemSwitchMethods for RingSetting { 
    fn init_content(_this: &mut ConfigBasicMenuItem) {
        let _value = CONFIG.lock().unwrap().ring;
    }

    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        let result = ConfigBasicMenuItem::change_key_value_i(CONFIG.lock().unwrap().ring, 0, 4, 1);
        if CONFIG.lock().unwrap().ring != result {
            CONFIG.lock().unwrap().ring = result;
            Self::set_help_text(this, None);
            Self::set_command_text(this, None);
            this.update_text();
            ring_change();
            // Update the config here by writing if the value changed.
            CONFIG.lock().unwrap().write();
            BasicMenuResult::se_cursor()
        } else {
            BasicMenuResult::new()
        }
    }

    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) {
        this.command_text = match CONFIG.lock().unwrap().ring {
            1 => "C".into(),
            2 => "B".into(),
            3 => "A".into(),
            4 => "S".into(),
            _ => "Default".into(),
        }
    }

    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) {
        if CONFIG.lock().unwrap().ring == 0 {
            this.help_text = "Rings will appear at normal rates and rarity.".into();
        } else {
            this.help_text = "Rings that appear will always have the set rarity.".into();
        }
    }
}

#[no_mangle]
extern "C" fn ring_callback() -> &'static mut ConfigBasicMenuItem {
    ConfigBasicMenuItem::new_switch::<RingSetting>("Bond Ring Rank")
}


pub fn ring_install() {
    cobapi::install_global_game_setting(ring_callback);
}


