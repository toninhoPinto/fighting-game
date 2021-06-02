use crate::{ecs_system::enemy_manager::EnemyManager, game_logic::characters::{Attack, player::Player}, scenes::overworld_scene::OverworldScene};

use super::Effect;

pub type CharacterEventActive = fn(&mut Player, &mut EnemyManager, &mut Effect) -> ();
pub type CharacterEvent = fn(&mut Player, &mut EnemyManager, i32, &mut Effect) -> ();
pub type CharacterEventUpdate = fn(&mut Player, &mut EnemyManager, i32, &mut Effect, f64,) -> ();
pub type CharacterEventMap = fn(&mut Player, &mut OverworldScene, &mut Effect) -> ();
pub type CharacterEventAttack = fn(&mut Player, &mut EnemyManager, i32, &mut Effect, &mut Attack) -> ();

#[derive(Clone)]
pub struct EventsPubSub {
    pub on_update: Vec<(CharacterEventUpdate, Effect)>,

    pub on_heal: Vec<(CharacterEvent, Effect)>,
    pub on_hurt: Vec<(CharacterEvent, Effect)>,
    pub on_death: Vec<(CharacterEvent, Effect)>,

    pub on_attack: Vec<(CharacterEvent, Effect)>,
    pub on_hit: Vec<(CharacterEventAttack, Effect)>,
    pub on_kill: Vec<(CharacterEvent, Effect)>,

    pub on_jump: Vec<(CharacterEvent, Effect)>,
    pub on_dash: Vec<(CharacterEvent, Effect)>,

    pub on_overworld_map: Vec<(CharacterEventMap, Effect)>,
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