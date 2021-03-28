use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::string::String;
use std::path::Path;

use directories::ProjectDirs;

use crate::game_logic::inputs::game_inputs::GameInputs;


//TODO: with controllers vs joysticks it might be needed to implement custom serialization 
//TODO into strings and you cannot use them as hashes unless they are stringified
pub fn load_controls() -> HashMap<String, GameInputs> {

    let proj_dir = ProjectDirs::from("com", "FightingGame",  "fighting game").unwrap();
    let config_dir = proj_dir.config_dir();
    let path = Path::new(config_dir).join("config_file.prefs.json");
    
    if let Err(e) = fs::create_dir_all(&config_dir) {println!("{:?} will continue with existing file", e)}
    
    
    if !path.exists()  {
        File::create(&path).unwrap();
    }

    let json_string = fs::read_to_string(&path).unwrap();

    if json_string.len() > 0 {
        let deserialized: HashMap<String, GameInputs> = serde_json::from_str(&json_string).unwrap();
            deserialized 
    } else {
        let mut controls: HashMap<String, GameInputs> = HashMap::new();
    
        controls.insert(0.to_string(), GameInputs::LightPunch);
        controls.insert(3.to_string(), GameInputs::MediumPunch);
        controls.insert(5.to_string(), GameInputs::HeavyPunch);
        controls.insert(1.to_string(), GameInputs::LightKick);
    
        controls.insert("U".to_string(), GameInputs::LightPunch);
        controls.insert("I".to_string(), GameInputs::MediumPunch);
        controls.insert("O".to_string(), GameInputs::HeavyPunch);
        controls.insert("J".to_string(), GameInputs::LightKick);
        
        controls.insert("W".to_string(), GameInputs::Vertical(1));
        controls.insert("S".to_string(), GameInputs::Vertical(-1));
        controls.insert("A".to_string(), GameInputs::Horizontal(-1));
        controls.insert("D".to_string(), GameInputs::Horizontal(1));
    
        // let _save_result =  fs::write(dirs::config_dir().unwrap()).unwrap();
    
        controls
    }


}