extern crate preferences;
use std::collections::HashMap;

use preferences::{AppInfo, PreferencesMap, Preferences};

const APP_INFO: AppInfo = AppInfo{name: "fighting game", author: "FightingGame"};
const PREFS_KEY: &str = "config_file";

use crate::game_logic::game_input::GameInputs;

pub fn load_controls() -> HashMap<std::string::String, GameInputs> {
    println!("Stored control configs in {:?}\\FightingGame", preferences::prefs_base_dir());
    //TODO: read first, if no file, then write for the first time

    let mut controls: PreferencesMap<String> = PreferencesMap::new();

    controls.insert(0.to_string(), GameInputs::LightPunch.to_string());
    controls.insert(3.to_string(), GameInputs::MediumPunch.to_string());
    controls.insert(5.to_string(), GameInputs::HeavyPunch.to_string());
    controls.insert(1.to_string(), GameInputs::LightKick.to_string());

    let save_result = controls.save(&APP_INFO, PREFS_KEY);

    PreferencesMap::load(&APP_INFO, PREFS_KEY).unwrap()
}