use std::collections::HashMap;

use super::{characters::player::Player, items::item_effects::{add_attack, apply_lifesteal, apply_poison_to_enemies}};

pub(crate) type ItemEffects = fn(&mut Player, &mut Effect) -> ();

pub mod events_pub_sub;

#[derive(Clone)]
pub struct Effect {
    //handler for function
    pub effect_id: i32,

    //for overtime effects
    pub duration: Option<i32>,
    pub time_elapsed: i32,
    pub change: Option<i32>,

    //for adding new animations
    pub add_attack: Option<String>,
}

impl Effect {
    pub fn new(effect_id: i32, duration:i32, change: i32) -> Self {
        Self {
            effect_id: effect_id,
            duration: Some(duration),
            time_elapsed: 0,
            change: Some(change),
            add_attack: None
        }
    }
}

pub fn hash_effects() -> HashMap<i32, ItemEffects>{
    let mut effects = HashMap::new();

    effects.insert(4, add_attack as ItemEffects);
    effects.insert(10, apply_poison_to_enemies as ItemEffects);
    effects.insert(9, apply_lifesteal as ItemEffects);

    effects
}