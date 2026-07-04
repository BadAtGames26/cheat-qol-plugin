use engage::{prelude::*, root::configbasicmenuitem::*};
use unity::prelude::*;

use crate::config::QOLCONFIG;
use engage::app::basicmenuitem::BasicMenuItem;

#[unity::inject(
    namespace = "BadCheats",
    name = "RewindSetting",
    parent = ConfigBasicMenuItem,
)]
pub struct RewindSetting{}


#[unity::injected_methods]
impl RewindSetting{
    #[override_virtual(name = "GetName")]
    pub fn get_name(self) -> Il2CppString {
       "Rewind Charges".into()
    }

    #[override_virtual(name = "ACall")]
    pub fn a_call(self) -> BasicMenuResult {
        BasicMenuResult::new()
    }

    #[override_virtual(name = "BuildAttribute")]
    pub fn build_attribute(self) -> BasicMenuItemAttribute {
        BasicMenuItemAttribute::enable()
    }

    #[override_virtual(name = "CustomCall")]
    pub fn custom_call(self) -> BasicMenuResult {
        let value = QOLCONFIG.lock().unwrap().rewind;
        let result = ConfigBasicMenuItem::change_key_value(value, 0, 6, 1);
        if value != result {
            QOLCONFIG.lock().unwrap().rewind = result;
            self.set_m_command_text(Self::get_command_text(result));
            self.update_text();
            // Update the config here by writing if the value changed.
            QOLCONFIG.lock().unwrap().write();
            BasicMenuResult::se_cursor()
        } else {
            BasicMenuResult::new()
        }
    }

    #[override_virtual(name = "InitContent")]
    pub fn init_content(self) {
        let value = QOLCONFIG.lock().unwrap().rewind;
        self.set_title_text(self.get_name());
        self.set_m_help_text("The number of rewinds that can be used in a battle.".into());
        self.set_m_command_text(Self::get_command_text(value));
        self.update_text();
        
    }

    #[override_virtual(name = "OnBuild")]
    pub fn on_build(self) {
        self.init_content();
    }
}

impl RewindSetting {
    pub fn get_command_text(value: i32) -> Il2CppString {
        match value {
            1 => "0",
            2 => "1",
            3 => "3",
            4 => "5",
            5 => "10",
            6 => "Unlimited",
            _ => "Default",
        }.into()
    }
}

pub fn register_rewind() -> Class {
    let result = cobapi::injection::register::<RewindSetting>();
    match result {
        Ok(t) => {
            t
        },
        Err(e) => panic!("Failed to register Rewinetting: {}.", e),
    }
}

#[no_mangle]
pub extern "C" fn rewind_callback() -> BasicMenuItem {
    let instance = RewindSetting::instantiate().unwrap();
    instance.try_cast::<BasicMenuItem>().unwrap()
}

//pub fn rewind_install() {
//    register_rewind();
//    cobapi::install_global_game_setting(rewind_callback);
//}