use engage::{prelude::*, root::configbasicmenuitem::*};
use unity::prelude::*;

use crate::config::QOLCONFIG;
use engage::app::basicmenuitem::BasicMenuItem;

#[unity::inject(
    namespace = "BadCheats",
    name = "ArenaSetting",
    parent = ConfigBasicMenuItem,
)]
pub struct ArenaSetting{}


#[unity::injected_methods]
impl ArenaSetting{
    #[override_virtual(name = "GetName")]
    pub fn get_name(self) -> Il2CppString {
       "Arena Matches".into()
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
        let value = QOLCONFIG.lock().unwrap().arenalimit;
        let result = ConfigBasicMenuItem::change_key_value(value, 0, 5, 1);
        if value != result {
            QOLCONFIG.lock().unwrap().arenalimit = result;
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
        let value = QOLCONFIG.lock().unwrap().arenalimit;
        self.set_title_text(self.get_name());
        self.set_m_help_text("The amount of battles that can be performed in the Arena.".into());
        self.set_m_command_text(Self::get_command_text(value));
        self.update_text();
        
    }

    #[override_virtual(name = "OnBuild")]
    pub fn on_build(self) {
        self.init_content();
    }
}

impl ArenaSetting {
    pub fn get_command_text(value: i32) -> Il2CppString {
        match value {
            1 => "0",
            2 => "5",
            3 => "10",
            4 => "100",
            5 => "1000",
            _ => "Default",
        }.into()
    }
}

pub fn register_arena() -> Class {
    let result = cobapi::injection::register::<ArenaSetting>();
    match result {
        Ok(t) => {
            t
        },
        Err(e) => panic!("Failed to register ArenaSetting: {}.", e),
    }
}

#[no_mangle]
pub extern "C" fn arena_callback() -> BasicMenuItem {
    let instance = ArenaSetting::instantiate().unwrap();
    instance.try_cast::<BasicMenuItem>().unwrap()
}

//pub fn arena_install() {
//    register_arena();
//    cobapi::install_global_game_setting(arena_callback);
//}