use engage::{prelude::*, root::configbasicmenuitem::*};
use unity::prelude::*;

use crate::config::QOLCONFIG;

#[unity::inject(
    namespace = "BadCheats",
    name = "RingSetting",
    parent = ConfigBasicMenuItem,
)]
pub struct RingSetting{}


#[unity::injected_methods]
impl RingSetting{
    #[override_virtual(name = "GetName")]
    pub fn get_name(self) -> Il2CppString {
       "Ring Stars".into()
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
        let value = QOLCONFIG.lock().unwrap().ring;
        let result = ConfigBasicMenuItem::change_key_value(value, 0, 4, 1);
        if value != result {
            QOLCONFIG.lock().unwrap().ring = result;
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
        let value = QOLCONFIG.lock().unwrap().ring;
        self.set_title_text(self.get_name());
        self.set_m_help_text("The rarity a bond ring will be.".into());
        self.set_m_command_text(Self::get_command_text(value));
        self.update_text();
        
    }

    #[override_virtual(name = "OnBuild")]
    pub fn on_build(self) {
        self.init_content();
    }
}

impl RingSetting {
    pub fn get_command_text(value: i32) -> Il2CppString {
        match value {
            1 => "C",
            2 => "B",
            3 => "A",
            4 => "S",
            _ => "Default",
        }.into()
    }
}

pub fn register_ring() -> Class {
    let result = cobapi::injection::register::<RingSetting>();
    match result {
        Ok(t) => {
            t
        },
        Err(_e) => panic!("Failed to register RingSetting."),
    }
}

#[no_mangle]
pub extern "C" fn ring_callback() -> ConfigBasicMenuItem {
    let instance = RingSetting::instantiate().unwrap();
    instance.try_cast::<ConfigBasicMenuItem>().unwrap()
}

pub fn ring_install() {
    register_ring();
    cobapi::install_global_game_setting(ring_callback);
}