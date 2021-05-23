use crate::{ecs_system::enemy_manager::EnemyManager, game_logic::characters::player::Player};

use super::Effect;


pub type CharacterEvent = fn(&mut Player, &mut EnemyManager, i32, &mut Effect) -> ();

#[derive(Clone)]
pub struct EventsPubSub {
    pub on_update: Vec<(CharacterEvent, Effect)>,

    pub on_heal: Vec<(CharacterEvent, Effect)>,
    pub on_hurt: Vec<(CharacterEvent, Effect)>,
    pub on_death: Vec<(CharacterEvent, Effect)>,

    pub on_attack: Vec<(CharacterEvent, Effect)>,
    pub on_hit: Vec<(CharacterEvent, Effect)>,
    pub on_kill: Vec<(CharacterEvent, Effect)>,

    pub on_jump: Vec<(CharacterEvent, Effect)>,
    pub on_dash: Vec<(CharacterEvent, Effect)>,

    pub on_overworld_map: Vec<(CharacterEvent, Effect)>,
    pub on_start_level: Vec<(CharacterEvent, Effect)>,
}

impl EventsPubSub {
    
    pub fn new() -> Self {
        Self{
            on_update: Vec::new(),

            on_heal: Vec::new(),
            on_hurt: Vec::new(),
            on_death: Vec::new(),
        
            on_attack: Vec::new(),
            on_hit: Vec::new(),
            on_kill: Vec::new(),
        
            on_jump: Vec::new(),
            on_dash:Vec::new(),
        
            on_overworld_map: Vec::new(),
            on_start_level: Vec::new(),
        }
    }
}