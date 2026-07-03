use engage::{BasicMenuItemAttribute, ProcVoidMethodExt, app::{BasicMenu, IBasicMenu, IBasicMenuMethods, IProcInst, IProcInstMethods}};
use engage::{root::ConfigMenu, system::collections::generic::IList_1Methods as _};
use engage::{BasicMenuResult, root::configbasicmenuitem::*};
use unity::prelude::*;
use engage::app::basicmenuitem::IBasicMenuItemMethods;
use engage::nn::hid::npadbutton::NpadButton;
use engage::{app::{pad::Pad, proc::Proc, procdesc::ProcDesc, procinst::ProcInst, procvoidmethod::ProcVoidMethod}};

use crate::{arena, discount, rewind, ring, summon, well};



#[unity::inject(
    namespace = "BadCheats",
    name = "Submenu",
    parent = ConfigBasicMenuItem,
)]
pub struct Submenu {}

#[unity::injected_methods]
impl Submenu {
    #[override_virtual(name = "CustomCall")]
    fn custom_call(self) -> BasicMenuResult {
        if !Pad::is_trigger(NpadButton::a()) {
            return BasicMenuResult::new();
        }
        let menu = self.get_menu();
        open_submenu(menu);
        BasicMenuResult::se_cursor()
    }

    #[override_virtual(name = "GetName")]
    pub fn get_name(self) -> Il2CppString {
       "Bad's Cheats".into()
    }

    #[override_virtual(name = "ACall")]
    pub fn a_call(self) -> BasicMenuResult {
        BasicMenuResult::new()
    }

    #[override_virtual(name = "BuildAttribute")]
    pub fn build_attribute(self) -> BasicMenuItemAttribute {
        BasicMenuItemAttribute::enable()
    }

    #[override_virtual(name = "InitContent")]
    pub fn init_content(self) {
        self.set_title_text(self.get_name());
        self.set_m_help_text("Open the settings menu for Cheats and QOL Plugin".into());
        self.set_m_command_text("Open".into());
        self.update_text();
        
    }

    #[override_virtual(name = "OnBuild")]
    pub fn on_build(self) {
        self.init_content();
        self.set_m_is_arrow(false);
        self.set_m_is_command_icon(true);
    }

}

extern "C" fn submenu_start(seq: ProcInst,  _method_info: OptionalMethod) {
    let parent = seq.m_super();
    if !parent.is_null() {
        unsafe { parent.cast::<BasicMenu>() }.set_active(false);
    }

    ConfigMenu::create_bind(seq);

    let child = seq.m_child();
    if child.is_null() {
        return;
    }
    let menu = unsafe { child.cast::<BasicMenu>() };
    menu.m_full_menu_item_list().clear();

    menu.m_full_menu_item_list().add(rewind::rewind_callback());
    menu.m_full_menu_item_list().add(well::well_callback());
    menu.m_full_menu_item_list().add(arena::arena_callback());
    menu.m_full_menu_item_list().add(summon::summon_callback());
    menu.m_full_menu_item_list().add(discount::discount_callback());
    menu.m_full_menu_item_list().add(ring::ring_callback());
}

extern "C" fn submenu_end(seq: ProcInst, _method_info: OptionalMethod) {
    let parent = seq.m_super();
    if parent.is_null() {
        return;
    }
    unsafe { parent.cast::<BasicMenu>() }.set_active(true);
}

pub fn open_submenu(parent: BasicMenu) {
    let seq = <ProcInst as FromIlInstance>::instantiate().expect("config submenu: ProcInst allocation failed");

    let steps = [
        Proc::label(0),
        Proc::call_2(ProcVoidMethod::from_fn(seq, submenu_start).expect("config submenu: ProcVoidMethod allocation failed")),
        Proc::call_2(ProcVoidMethod::from_fn(seq, submenu_end).expect("config submenu: ProcVoidMethod allocation failed")),
        Proc::end(),
    ];
    let descs = Array::<ProcDesc>::new(<ProcDesc as ClassIdentity>::class().raw(), steps.len()).expect("config submenu: ProcDesc[] allocation failed");
    for (i, d) in steps.iter().enumerate() {
        descs.set(i, *d);
    }

    seq.create_bind(parent, descs, Il2CppString::from("BadCheatsSubmenu"));
}

pub fn register_submenu() -> Class {
    let result = cobapi::injection::register::<Submenu>();
    match result {
        Ok(t) => {
            t
        },
        Err(_e) => panic!("Failed to register RingSetting."),
    }
}

#[no_mangle]
pub extern "C" fn submenu_callback() -> ConfigBasicMenuItem {
    let instance = Submenu::instantiate().unwrap();
    instance.try_cast::<ConfigBasicMenuItem>().unwrap()
}

pub fn submenu_install() {
    register_submenu();
    cobapi::install_global_game_setting(submenu_callback);
    cobapi::install_game_setting(submenu_callback);
}