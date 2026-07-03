use engage::{prelude::*, root::{ConfigBasicMenuItem_ConfigMethodKind, configbasicmenuitem::*}};
use unity::prelude::*;

use crate::config::QOLCONFIG;
use engage::root::configbasicmenuitem::ConfigBasicMenuItem;
use engage::app::basicmenuitem::BasicMenuItem;

#[unity::inject(
    namespace = "BadCheats",
    name = "DiscountSetting",
    parent = ConfigBasicMenuItem,
)]
pub struct DiscountSetting{}


#[unity::injected_methods]
impl DiscountSetting{
    #[override_virtual(name = "GetName")]
    pub fn get_name(self) -> Il2CppString {
       "Discount Charges".into()
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
        let value = QOLCONFIG.lock().unwrap().discount;
        let result = ConfigBasicMenuItem::change_key_value_2(value, 0.0, 1.0, 0.1);
        if value != result {
            let round = (result * 10.0).round() / 10.0;
            QOLCONFIG.lock().unwrap().discount = round;
            self.set_m_help_text(Self::get_help_text(round));
            self.set_m_gauge_ratio(round);
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
        let value = QOLCONFIG.lock().unwrap().discount;
        self.set_title_text(self.get_name());
        self.set_m_help_text(Self::get_help_text(value));
        self.set_m_gauge_ratio(value);
        self.update_text();
    }

    #[override_virtual(name = "OnBuild")]
    pub fn on_build(self) {
        self.set_m_config_method(ConfigBasicMenuItem_ConfigMethodKind::gauge());
        self.init_content();
    }
}

impl DiscountSetting {
    pub fn get_help_text(value: f32) -> Il2CppString {
        format!("Silver Card Discount: {}%", (value)).into()
    }
}

pub fn register_discount() -> Class {
    let result = cobapi::injection::register::<DiscountSetting>();
    match result {
        Ok(t) => {
            t
        },
        Err(_e) => panic!("Failed to register DiscountSetting."),
    }
}

#[no_mangle]
pub extern "C" fn discount_callback() -> BasicMenuItem {
    let instance = DiscountSetting::instantiate().unwrap();
    instance.try_cast::<BasicMenuItem>().unwrap()
}

//pub fn discount_install() {
//    register_discount();
//    cobapi::install_global_game_setting(discount_callback);
//}