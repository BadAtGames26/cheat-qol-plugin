use engage::menu::{config::{ConfigBasicMenuItem, ConfigBasicMenuItemSwitchMethods}, BasicMenuResult};
use unity::prelude::*;

use crate::{CONFIG, event::*};

pub struct SummonSetting;

impl ConfigBasicMenuItemSwitchMethods for SummonSetting { 
    fn init_content(_this: &mut ConfigBasicMenuItem) {
        let _value = CONFIG.lock().unwrap().summon;
    }

    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        let result = ConfigBasicMenuItem::change_key_value_i(CONFIG.lock().unwrap().summon, 0, 3, 1);
        if CONFIG.lock().unwrap().summon != result {
            CONFIG.lock().unwrap().summon = result;
            Self::set_help_text(this, None);
            Self::set_command_text(this, None);
            this.update_text();
            summon_change();
            // Update the config here by writing if the value changed.
            CONFIG.lock().unwrap().write();
            BasicMenuResult::se_cursor()
        } else {
            BasicMenuResult::new()
        }
    }

    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) {
        this.command_text = match CONFIG.lock().unwrap().summon {
            1 => "3*".into(),
            2 => "4*".into(),
            3 => "5*".into(),
            _ => "Default".into(),
        }
    }

    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) {
        if CONFIG.lock().unwrap().summon == 0 {
            this.help_text = "Summons will appear at normal rates and ranks.".into();
        } else {
            this.help_text = "Summons that appear will always have the set star rank.".into();
        }
    }
}

#[no_mangle]
extern "C" fn summon_callback() -> &'static mut ConfigBasicMenuItem {
    ConfigBasicMenuItem::new_switch::<SummonSetting>("Summon Star Rank")
}


pub fn summon_install() {
    cobapi::install_global_game_setting(summon_callback);
}


