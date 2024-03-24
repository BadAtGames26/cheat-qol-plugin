use engage::{proc::ProcInst, menu::{*, config::{ConfigBasicMenuItemCommandMethods, ConfigBasicMenuItem}}, util::get_instance, pad::Pad};
use unity::prelude::*;

use crate::{arena::ArenaLimitSetting, emblem::EmblemLeaveSetting, rewind::RewindSetting, ring::RingSetting, silvercard::DiscountSetting, summon::SummonSetting, well::WellSetting};


pub struct Submenu;

impl ConfigBasicMenuItemCommandMethods for Submenu {
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        let pad_instance = get_instance::<Pad>();

        // Check if A is pressed before executing any of this
        if pad_instance.npad_state.buttons.a() {
            if !pad_instance.old_buttons.a() {
                // Close the original Settings menu temporarily so it doesn't get drawn in the background
                this.menu.close_anime_all();

                // Initialize the menu
                ConfigMenu::create_bind(this.menu);
                
                let config_menu = this.menu.proc.child.cast_mut::<BasicMenu<ConfigBasicMenuItem>>();

                // Register a OnDispose callback to restore the previous menu
                config_menu
                    .get_class_mut()
                    .get_virtual_method_mut("OnDispose")
                    .map(|method| method.method_ptr = open_anime_all_ondispose as _)
                    .unwrap();
                
                // Clear the buttons in the List so we can add our own
                config_menu.full_menu_item_list.clear();

                config_menu.add_item(ConfigBasicMenuItem::new_switch::<EmblemLeaveSetting>("Emblem Leave"));
                config_menu.add_item(ConfigBasicMenuItem::new_switch::<ArenaLimitSetting>("Arena Battle Limit"));
                config_menu.add_item(ConfigBasicMenuItem::new_switch::<RewindSetting>("Rewind Charges"));
                config_menu.add_item(ConfigBasicMenuItem::new_switch::<SummonSetting>("Summon Rank"));
                config_menu.add_item(ConfigBasicMenuItem::new_switch::<RingSetting>("Ring Rank"));
                config_menu.add_item(ConfigBasicMenuItem::new_gauge::<DiscountSetting>("Silver Card Discount"));
                config_menu.add_item(ConfigBasicMenuItem::new_switch::<WellSetting>("Well Rank"));
                
                BasicMenuResult::se_cursor()
            } else {
                BasicMenuResult::new()
            }
        } else {
            BasicMenuResult::new()
        }
    }

    extern "C" fn set_command_text(this: &mut engage::menu::config::ConfigBasicMenuItem, _method_info: unity::prelude::OptionalMethod) {
        this.command_text = "Open".into();
    }

    extern "C" fn set_help_text(this: &mut engage::menu::config::ConfigBasicMenuItem, _method_info: unity::prelude::OptionalMethod) {
        this.help_text = "Open the settings menu for Cheats and QOL Plugin".into();
    }
}

#[no_mangle]
extern "C" fn submenu_callback() -> &'static mut ConfigBasicMenuItem {
    ConfigBasicMenuItem::new_command::<Submenu>("Cheats and QOL")
}

pub fn submenu_install() {
    cobapi::install_global_game_setting(submenu_callback);
}

// This is from the engage crate, its not a public function so we copy it here to use it
extern "C" fn open_anime_all_ondispose(this: &mut ProcInst, _method_info: OptionalMethod) {
    this.parent.get_class().get_virtual_method("OpenAnimeAll").map(|method| {
        let open_anime_all = unsafe { std::mem::transmute::<_, extern "C" fn(&ProcInst, &MethodInfo)>(method.method_info.method_ptr) };
        open_anime_all(this.parent, method.method_info);
    });
}
