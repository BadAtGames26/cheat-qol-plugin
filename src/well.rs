use engage::menu::{config::{ConfigBasicMenuItem, ConfigBasicMenuItemSwitchMethods}, BasicMenuResult};
use unity::prelude::*;

use crate::{config::CONFIG, event::*};

pub struct WellSetting;

impl ConfigBasicMenuItemSwitchMethods for WellSetting { 
    fn init_content(_this: &mut ConfigBasicMenuItem) {
        let _value = CONFIG.lock().unwrap().well;
    }

    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        let result = ConfigBasicMenuItem::change_key_value_i(CONFIG.lock().unwrap().well, 0, 5, 1);
        if CONFIG.lock().unwrap().well != result {
            CONFIG.lock().unwrap().well = result;
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
        this.command_text = match CONFIG.lock().unwrap().well {
            1 => "1*".into(),
            2 => "2*".into(),
            3 => "3*".into(),
            4 => "4*".into(),
            5 => "5*".into(),
            _ => "Default".into(),
        }
    }

    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) {
        this.help_text = "The star rating the Somniel well will be.".into();
    }
}

/* #[no_mangle]
extern "C" fn well_callback() -> &'static mut ConfigBasicMenuItem {
    ConfigBasicMenuItem::new_switch::<WellSetting>("Well Star Rating")
}

pub fn well_install() {
    cobapi::install_global_game_setting(well_callback);
} */