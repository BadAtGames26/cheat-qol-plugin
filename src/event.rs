use cobapi::{SystemEvent, Event};
use engage::gamedata::Gamedata;

use crate::{config::CONFIG, gameparam::GameParam};

// Change the values when the language is loaded, this is after Gamedata is loaded, so no issues occur there. Reloading Gamedata might revert these changes.
#[no_mangle]
extern "C" fn listener(event: &Event<SystemEvent>) {
    if let Event::Args(ev) = event {
        if let SystemEvent::LanguageChanged = ev {
            arenalimit_change();
            rewind_change();
            summon_change();
            ring_change();
            discount_change();
        }
    }
}

pub fn arenalimit_change() {
    // Arena:Battle Limmit
    let param = GameParam::get_mut("闘技場:回数制限");
    if param.is_some() {
        let param = param.unwrap();
        param.value = match CONFIG.lock().unwrap().arenalimit  {
            1 => 0.0,
            2 => 5.0,
            3 => 10.0,
            4 => 100.0,
            5 => 1000.0,
            _ => 3.0,

        };
        println!("Setting number of arena battles to {}", param.value);
    }
}

pub fn rewind_change() {
    // The three params that control rewinds for Normal, Hard and Lunatic
    let paramlist = ["巻き戻し最大回数ノーマル", "巻き戻し最大回数ハード", "巻き戻し最大回数ルナティック"];
    for item in paramlist {
        let param = GameParam::get_mut(item);
        if param.is_some() {
            let param = param.unwrap();
            param.value = match CONFIG.lock().unwrap().rewind  {
                1 => 0.0,
                2 => 10.0,
                3 => -1.0,
                _ => param.initial,
    
            };
            println!("Setting number of rewinds to {}", param.value);
        }
    }
}

pub fn summon_change() {
    // The three params that control the appearance rates for 3, 4 and 5 star summons
    let paramlist = ["出現率★１", "出現率★２", "出現率★３"];
    let param1 = GameParam::get_mut(paramlist[0]).unwrap();
    let param2 = GameParam::get_mut(paramlist[1]).unwrap();
    let param3 = GameParam::get_mut(paramlist[2]).unwrap();
    match CONFIG.lock().unwrap().summon {
        // Setting the appearance rate to 100 for the rarity we want to appear and the other two to 0
        1 => {
            param1.value = 100.0;
            param2.value = 0.0;
            param3.value = 0.0;
            println!("Only 3* summons will appear");
        }
        2 => {
            param1.value = 0.0;
            param2.value = 100.0;
            param3.value = 0.0;
            println!("Only 4* summons will appear");

        }
        3 => {
            param1.value = 0.0;
            param2.value = 0.0;
            param3.value = 100.0;
            println!("Only 5* summons will appear");
        }
        _ => {
            param1.value = param1.initial;
            param2.value = param2.initial;
            param3.value = param3.initial;  
        }
    }
}

pub fn ring_change() {
    // The three params that control the appearance rates for B, A and S rings
    let paramlist = ["指輪精製確率B", "指輪精製確率A", "指輪精製確率S"];
    let param1 = GameParam::get_mut(paramlist[0]).unwrap();
    let param2 = GameParam::get_mut(paramlist[1]).unwrap();
    let param3 = GameParam::get_mut(paramlist[2]).unwrap();
    match CONFIG.lock().unwrap().ring {
        // Setting the appearance rate to 100 for the rarity we want to appear and the other two to 0, for C rarity all are set to 0
        1 => {
            param1.value = 0.0;
            param2.value = 0.0;
            param3.value = 0.0;
            println!("Only C rings will appear");
        }
        2 => {
            param1.value = 100.0;
            param2.value = 0.0;
            param3.value = 0.0;
            println!("Only B rings will appear");

        }
        3 => {
            param1.value = 0.0;
            param2.value = 100.0;
            param3.value = 0.0;
            println!("Only A rings will appear");
        }
        4 => {
            param1.value = 0.0;
            param2.value = 0.0;
            param3.value = 100.0;
            println!("Only S rings will appear");
        }
        _ => {
            param1.value = param1.initial;
            param2.value = param2.initial;
            param3.value = param3.initial;  
        }
    }
}

pub fn discount_change() {
    // Silver Card Discount Rate
    let param = GameParam::get_mut("シルバーカード割引率").unwrap();
    param.value =CONFIG.lock().unwrap().discount;
    println!("Setting silver card discount rate to {}", param.value);
}

// This is just a function to use in the main function since it did not like calling the listener function from here
pub fn listener_install() {
    cobapi::register_system_event_handler(listener);
}