use std::collections::HashMap;

use super::{characters::player::Player, items::item_effects::{add_attack, apply_add_attack_at_level_start, apply_anti_grav, apply_life_on_kill, apply_lifesteal, apply_map_exploration, apply_once_in_awhile_forget_or_remenber_attacks, apply_poison_to_enemies, apply_remove_all_extra_attacks_on_hurt, change_stats, heal_on_active, remove_all_extra_punches}};

pub(crate) type ItemEffects = fn(&mut Player, &mut Effect) -> ();

pub mod events_pub_sub;

#[derive(Clone)]
pub struct Effect {
    //handler for function
    pub effect_id: i32,

    //for overtime effects
    pub duration: Option<i32>,
    pub time_elapsed: i32,
    pub apply_at_every: Option<i32>,
    pub change: Option<i32>,
    pub stat:  Option<Vec<String>>,

    //for adding new animations
    pub add_attack: Option<String>,
}


pub fn hash_effects() -> HashMap<i32, ItemEffects>{
    let mut effects = HashMap::new();

    effects.insert(4, add_attack as ItemEffects);
    effects.insert(5, apply_add_attack_at_level_start as ItemEffects);
    effects.insert(6, apply_once_in_awhile_forget_or_remenber_attacks as ItemEffects);
    effects.insert(8, apply_life_on_kill as ItemEffects);
    effects.insert(9, apply_lifesteal as ItemEffects);
    effects.insert(10,apply_poison_to_enemies as ItemEffects);
    effects.insert(11,apply_remove_all_extra_attacks_on_hurt as ItemEffects);
    effects.insert(12, apply_anti_grav as ItemEffects);
    effects.insert(14, change_stats as ItemEffects);
    effects.insert(22, apply_map_exploration as ItemEffects);
    effects.insert(27,remove_all_extra_punches as ItemEffects);
    effects.insert(34, heal_on_active as ItemEffects);

    effects
}