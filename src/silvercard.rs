use engage::menu::{config::{ConfigBasicMenuItem, ConfigBasicMenuItemGaugeMethods}, BasicMenuResult};
use unity::prelude::*;

use crate::{config::CONFIG, event::*};

pub struct DiscountSetting;

impl ConfigBasicMenuItemGaugeMethods for DiscountSetting { 
    fn init_content(this: &mut ConfigBasicMenuItem) {
        this.gauge_ratio = CONFIG.lock().unwrap().discount;
    }

    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        let result = ConfigBasicMenuItem::change_key_value_f(CONFIG.lock().unwrap().discount, 0.0, 1.0, 0.1);
        if CONFIG.lock().unwrap().discount != result {
            CONFIG.lock().unwrap().discount = result;
            this.gauge_ratio = result;
            Self::set_help_text(this, None);
            this.update_text();
            discount_change();
            // Update the config here by writing if the value changed.
            CONFIG.lock().unwrap().write();
            BasicMenuResult::se_cursor()
        } else {
            BasicMenuResult::new()
        }
    }

    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) {
        this.help_text = format!("Silver Card Discount: {}%", (this.gauge_ratio*100.0) as i32).into();
    }
}

#[no_mangle]
extern "C" fn discount_callback() -> &'static mut ConfigBasicMenuItem {
    ConfigBasicMenuItem::new_gauge::<DiscountSetting>("Silver Card Discount Rate")
}


pub fn discount_install() {
    cobapi::install_global_game_setting(discount_callback);
}


