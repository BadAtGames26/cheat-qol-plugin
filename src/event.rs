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

// This is just a function to use in the main function since it did not like calling the listener function from here
pub fn listener_install() {
    cobapi::register_system_event_handler(listener);
}