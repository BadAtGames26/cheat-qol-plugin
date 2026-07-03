use engage::{prelude::*, root::configbasicmenuitem::*};
use unity::prelude::*;

use crate::config::QOLCONFIG;

#[unity::inject(
    namespace = "BadCheats",
    name = "SummonSetting",
    parent = ConfigBasicMenuItem,
)]
pub struct SummonSetting{}


#[unity::injected_methods]
impl SummonSetting{
    #[override_virtual(name = "GetName")]
    pub fn get_name(self) -> Il2CppString {
       "Summon Stars".into()
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
        let value = QOLCONFIG.lock().unwrap().summon;
        let result = ConfigBasicMenuItem::change_key_value(value, 0, 3, 1);
        if value != result {
            QOLCONFIG.lock().unwrap().summon = result;
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
        let value = QOLCONFIG.lock().unwrap().summon;
        self.set_title_text(self.get_name());
        self.set_m_help_text("The stars a summon will have.".into());
        self.set_m_command_text(Self::get_command_text(value));
        self.update_text();
        
    }

    #[override_virtual(name = "OnBuild")]
    pub fn on_build(self) {
        self.init_content();
    }
}

impl SummonSetting {
    pub fn get_command_text(value: i32) -> Il2CppString {
        match value {
            1 => "3*",
            2 => "4*",
            3 => "5*",
            _ => "Default",
        }.into()
    }
}

pub fn register_summon() -> Class {
    let result = cobapi::injection::register::<SummonSetting>();
    match result {
        Ok(t) => {
            t
        },
        Err(_e) => panic!("Failed to register SummonSetting."),
    }
}

#[no_mangle]
pub extern "C" fn summon_callback() -> ConfigBasicMenuItem {
    let instance = SummonSetting::instantiate().unwrap();
    instance.try_cast::<ConfigBasicMenuItem>().unwrap()
}

pub fn summon_install() {
    register_summon();
    cobapi::install_global_game_setting(summon_callback);
}