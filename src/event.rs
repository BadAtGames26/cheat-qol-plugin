use cobapi::{SystemEvent, Event};

use crate::config::QOLCONFIG;
use engage::app::{IGameParamMethods, IProcInstMethods, IStructData_1Methods, gameparam::GameParam};

// Change the values when the language is loaded, this is after Gamedata is loaded, so no issues occur there. Reloading Gamedata might revert these changes.
#[no_mangle]
extern "C" fn listener(event: &Event<SystemEvent>) {
    if let Event::Args(ev) = event {
        if let SystemEvent::ProcInstJump { proc, label } = ev {
            // Loads the config values around the time the save is loaded and the running sprites appear
            // MainMainSequence, 29 = Select Save, 6 = Select New Game
            if proc.get_hash_code() == -1912552174 && (*label == 29 || *label == 6) {
                println!("Attempting to change settings for Cheats/QOL Plugin");
                rewind_change();
                well_change();
                arenalimit_change();
                summon_change();
                discount_change();
                ring_change();
            }
        }
    }
}

pub fn rewind_change() {
    // The three params that control rewinds for Normal, Hard and Lunatic
    let paramlist = ["巻き戻し最大回数ノーマル", "巻き戻し最大回数ハード", "巻き戻し最大回数ルナティック"];
    for item in paramlist {
        let param = GameParam::get(item.into());
        let value = match QOLCONFIG.lock().unwrap().rewind  {
            1 => 0.0,
            2 => 1.0,
            3 => 3.0,
            4 => 5.0,
            5 => 10.0,
            6 => -1.0,
            _ => param.get_initial(),
    
        };
        param.set_value(value);
        let difficulty = match item {
            "巻き戻し最大回数ノーマル" => "Normal",
            "巻き戻し最大回数ハード" => "Hard",
            "巻き戻し最大回数ルナティック" => "Maddening",
            _ => "Unknown",
        };
        println!("Setting number of {} rewinds to {}", difficulty, param.get_value() );
    }
}

pub fn well_change() {
    let param1 = GameParam::get("井戸期待度２必要価値".into());
    let param2 = GameParam::get("井戸期待度３必要価値".into());
    let param3 = GameParam::get("井戸期待度４必要価値".into());
    let param4 = GameParam::get("井戸期待度５必要価値".into());
    match QOLCONFIG.lock().unwrap().well {
        // Setting the cost for all well rankings to 1 million except for the one we want, so its still possible to get that ranking, but unlikely in normal gameplay.
        1 => {
            param1.set_value(1000000.0);
            param2.set_value(1000000.0);
            param3.set_value(1000000.0);
            param4.set_value(1000000.0);

            println!("Well will be 1*");
        }
        2 => {
            param1.set_value(0.0);
            param2.set_value(1000000.0);
            param3.set_value(1000000.0);
            param4.set_value(1000000.0);

            println!("Well will be 2*");

        }
        3 => {
            param1.set_value(1000000.0);
            param2.set_value(0.0);
            param3.set_value(1000000.0);
            param4.set_value(1000000.0);
            
            println!("Well will be 3*");
        }
        4 => {
            param1.set_value(1000000.0);
            param2.set_value(1000000.0);
            param3.set_value(0.0);
            param4.set_value(1000000.0);
            
            println!("Well will be 4*");
        }
        5 => {
            param1.set_value(1000000.0);
            param2.set_value(1000000.0);
            param3.set_value(1000000.0);
            param4.set_value(0.0);
            println!("Well will be 5*");
        }
        _ => {
            param1.set_value(param1.get_initial());
            param2.set_value(param2.get_initial());
            param3.set_value(param3.get_initial());
            param4.set_value(param4.get_initial());
            println!("Well will act according to default settings");
        }
    }

}

pub fn arenalimit_change() {
    // Arena:Battle Limmit
    let param = GameParam::get("闘技場:回数制限".into());
    let value = match QOLCONFIG.lock().unwrap().arenalimit  {
            1 => 0.0,
            2 => 5.0,
            3 => 10.0,
            4 => 100.0,
            5 => 1000.0,
            _ => param.get_initial(),
    };
    param.set_value(value);
    println!("Setting number of arena battles to {}", param.get_value());
}

pub fn summon_change() {
    // The three params that control the appearance rates for 3, 4 and 5 star summons
    let paramlist = ["出現率★１".into(), "出現率★２".into(), "出現率★３".into()];
    let param1 = GameParam::get(paramlist[0]);
    let param2 = GameParam::get(paramlist[1]);
    let param3 = GameParam::get(paramlist[2]);
    match QOLCONFIG.lock().unwrap().summon {
        // Setting the appearance rate to 100 for the rarity we want to appear and the other two to 0
        1 => {
            param1.set_value(100.0);
            param2.set_value(0.0);
            param3.set_value(0.0);
            println!("Only 3* summons will appear");
        }
        2 => {
            param1.set_value(0.0);
            param2.set_value(100.0);
            param3.set_value(0.0);
            println!("Only 4* summons will appear");

        }
        3 => {
            param1.set_value(0.0);
            param2.set_value(0.0);
            param3.set_value(100.0);
            println!("Only 5* summons will appear");
        }
        _ => {
            param1.set_value(param1.get_initial());
            param2.set_value(param2.get_initial());
            param3.set_value(param3.get_initial());
            println!("Any * summons will appear");
        }
    }
}

pub fn discount_change() {
    // Silver Card Discount Rate
    let param = GameParam::get("シルバーカード割引率".into());
    // Some math to round numbers evenly
    let value = (QOLCONFIG.lock().unwrap().discount*10.0).round() / 10.0;
    param.set_value(value);
    println!("Setting silver card discount rate to {:.1}", value);
}

pub fn ring_change() {
    // The three params that control the appearance rates for B, A and S rings
    let paramlist = ["指輪精製確率B".into(), "指輪精製確率A".into(), "指輪精製確率S".into()];
    let param1 = GameParam::get(paramlist[0]);
    let param2 = GameParam::get(paramlist[1]);
    let param3 = GameParam::get(paramlist[2]);
    match QOLCONFIG.lock().unwrap().ring {
        // Setting the appearance rate to 100 for the rarity we want to appear and the other two to 0, for C rarity all are set to 0
        1 => {
            param1.set_value(0.0);
            param2.set_value(0.0);
            param3.set_value(0.0);
            println!("Only C rings will appear");
        }
        2 => {
            param1.set_value(100.0);
            param2.set_value(0.0);
            param3.set_value(0.0);
            println!("Only B rings will appear");

        }
        3 => {
            param1.set_value(0.0);
            param2.set_value(100.0);
            param3.set_value(0.0);
            println!("Only A rings will appear");
        }
        4 => {
            param1.set_value(0.0);
            param2.set_value(0.0);
            param3.set_value(100.0);
            println!("Only S rings will appear");
        }
        _ => {
            param1.set_value(param1.get_initial());
            param2.set_value(param2.get_initial());
            param3.set_value(param3.get_initial());
            println!("Any rings will appear");
        }
     }
}

// This is just a function to use in the main function since it did not like calling the listener function from here
pub fn listener_install() {
    cobapi::register_system_event_handler(listener);
}