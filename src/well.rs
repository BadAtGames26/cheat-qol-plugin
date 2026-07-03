use engage::{prelude::*, root::configbasicmenuitem::*};
use unity::prelude::*;

use crate::config::QOLCONFIG;

#[unity::inject(
    namespace = "BadCheats",
    name = "WellSetting",
    parent = ConfigBasicMenuItem,
)]
pub struct WellSetting{}


#[unity::injected_methods]
impl WellSetting{
    #[override_virtual(name = "GetName")]
    pub fn get_name(self) -> Il2CppString {
       "Well Stars".into()
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
        let value = QOLCONFIG.lock().unwrap().well;
        let result = ConfigBasicMenuItem::change_key_value(value, 0, 5, 1);
        if value != result {
            QOLCONFIG.lock().unwrap().well = result;
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
        let value = QOLCONFIG.lock().unwrap().well;
        self.set_title_text(self.get_name());
        self.set_m_help_text("The star rating the Somniel well will be.".into());
        self.set_m_command_text(Self::get_command_text(value));
        self.update_text();
        
    }

    #[override_virtual(name = "OnBuild")]
    pub fn on_build(self) {
        self.init_content();
    }
}

impl WellSetting {
    pub fn get_command_text(value: i32) -> Il2CppString {
        match value {
            1 => "1*",
            2 => "2*",
            3 => "3*",
            4 => "4*",
            5 => "5*",
            _ => "Default",
        }.into()
    }
}

pub fn register_well() -> Class {
    let result = cobapi::injection::register::<WellSetting>();
    match result {
        Ok(t) => {
            t
        },
        Err(_e) => panic!("Failed to register WellSetting."),
    }
}

#[no_mangle]
pub extern "C" fn well_callback() -> ConfigBasicMenuItem {
    let instance = WellSetting::instantiate().unwrap();
    instance.try_cast::<ConfigBasicMenuItem>().unwrap()
}

pub fn well_install() {
    register_well();
    cobapi::install_global_game_setting(well_callback);
}