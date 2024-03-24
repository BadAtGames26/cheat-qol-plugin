use cobapi::{SystemEvent, Event};
use engage::gamedata::Gamedata;

use crate::{config::CONFIG, gameparam::GameParam};

// Change the values when the language is loaded, this is after Gamedata is loaded, so no issues occur there. Reloading Gamedata might revert these changes.
#[no_mangle]
extern "C" fn listener(event: &Event<SystemEvent>) {
    if let Event::Args(ev) = event {
        if let SystemEvent::ProcInstJump { proc, label } = ev {
            // Loads the config values around the time the save is loaded and the running sprites appear
            if proc.hashcode == -1118443598 && *label == 0 {
                println!("Attempting to change settings for Cheats/QOL Plugin");
                arenalimit_change();
                rewind_change();
                summon_change();
                ring_change();
                discount_change();
                well_change();
            }
        }
    }
}

pub fn arenalimit_change() {
    // Arena:Battle Limmit
    let param = GameParam::get_mut("闘技場:回数制限").unwrap();
    param.value = match CONFIG.lock().unwrap().arenalimit  {
            1 => 0.0,
            2 => 5.0,
            3 => 10.0,
            4 => 100.0,
            5 => 1000.0,
            _ => param.initial,
    };
    println!("Setting number of arena battles to {}", param.value);
}

pub fn rewind_change() {
    // The three params that control rewinds for Normal, Hard and Lunatic
    let paramlist = ["巻き戻し最大回数ノーマル", "巻き戻し最大回数ハード", "巻き戻し最大回数ルナティック"];
    for item in paramlist {
        let param = GameParam::get_mut(item).unwrap();
        param.value = match CONFIG.lock().unwrap().rewind  {
            1 => 0.0,
            2 => 10.0,
            3 => -1.0,
            _ => param.initial,
    
        };
        println!("Setting number of rewinds to {}", param.value);
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
            println!("Rings will appear at normal rates");
  
        }
    }
}

pub fn discount_change() {
    // Silver Card Discount Rate
    let param = GameParam::get_mut("シルバーカード割引率").unwrap();
    // Some math to round numbers evenly
    param.value = (CONFIG.lock().unwrap().discount*10.0).round() / 10.0;
    println!("Setting silver card discount rate to {:.1}", param.value);
}

pub fn well_change() {
    let param1 = GameParam::get_mut("井戸期待度２必要価値").unwrap();
    let param2 = GameParam::get_mut("井戸期待度３必要価値").unwrap();
    let param3 = GameParam::get_mut("井戸期待度４必要価値").unwrap();
    let param4 = GameParam::get_mut("井戸期待度５必要価値").unwrap();
    match CONFIG.lock().unwrap().well {
        // Setting the cost for all well rankings to 1 million except for the one we want, so its still possible to get that ranking, but unlikely in normal gameplay.
        1 => {
            param1.value = 1000000.0;
            param2.value = 1000000.0;
            param3.value = 1000000.0;
            param4.value = 1000000.0;
            println!("Well will be 1*");
        }
        2 => {
            param1.value = 0.0;
            param2.value = 1000000.0;
            param3.value = 1000000.0;
            param4.value = 1000000.0;
            println!("Well will be 2*");

        }
        3 => {
            param1.value = 1000000.0;
            param2.value = 0.0;
            param3.value = 100000.0;
            param4.value = 1000000.0;
            println!("Well will be 3*");
        }
        4 => {
            param1.value = 1000000.0;
            param2.value = 1000000.0;
            param3.value = 0.0;
            param3.value = 1000000.0;
            println!("Well will be 4*");
        }
        5 => {
            param1.value = 1000000.0;
            param2.value = 1000000.0;
            param3.value = 1000000.0;
            param4.value = 0.0;
            println!("Well will be 5*");
        }
        _ => {
            param1.value = param1.initial;
            param2.value = param2.initial;
            param3.value = param3.initial;
            param4.value = param4.initial; 
            println!("Well will act according to default settings");
        }
    }

}

// This is just a function to use in the main function since it did not like calling the listener function from here
pub fn listener_install() {
    cobapi::register_system_event_handler(listener);
}