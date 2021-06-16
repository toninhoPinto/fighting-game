use std::collections::HashMap;

use crate::{asset_management::rng_tables::LootTable, game_logic::characters::Character};


pub fn change_spawn_item(items: Vec<i32>, chance_mod: i32, character: &Character, loot_tables: &mut HashMap<String, LootTable>) {
    for (_table_type, table) in loot_tables.iter_mut() { 
        for item in table.items.iter_mut() {
            if items.contains(&(item.item_id as i32)) {
                item.rarity *= chance_mod as u64;
            }
        }
    }
}

pub fn stop_spawn_item(items: Vec<i32>, _: i32, _: &Character, loot_tables: &mut HashMap<String, LootTable>) {
    for (_table_type, table) in loot_tables.iter_mut() {
        table.items.retain(|item| {!items.contains(&(item.item_id as i32))});
        table.acc = table.items.iter().map(|i|{i.rarity}).sum();
    }
}


pub fn stop_attack_spawn(_: Vec<i32>, _: i32, character: &Character, loot_tables: &mut HashMap<String, LootTable>) {

    for (_table_type, table) in loot_tables.iter_mut() { 

        table.items.retain(|item| {
                let max_punches_already = item.item_id == 4 && character.punch_string_curr >= character.punch_string_max;

                let max_kicks_already = item.item_id == 5 && character.kick_string_curr >= character.kick_string_max;
                let max_air_punches_already = item.item_id == 6 && character.airborne_punch_string_curr >= character.airborne_punch_string_max;
                let max_air_kicks_already = item.item_id == 7 && character.airborne_kick_string_curr >= character.airborne_kick_string_max;

                let launch_already = item.item_id == 8 && character.directional_attacks_mask_curr & 0b0001u32 > 0;
                let dropper_already = item.item_id == 9 && character.directional_attacks_mask_curr & 0b0010u32 > 0;
                let dashing_already = item.item_id == 10 && character.directional_attacks_mask_curr & 0b0100u32 > 0;
                let crash_already = item.item_id == 11 && character.directional_attacks_mask_curr & 0b1000u32 > 0;

                let all_attacks_mastered = 
                    character.punch_string_curr >= character.punch_string_max && 
                    character.kick_string_curr >= character.kick_string_max && 
                    character.airborne_punch_string_curr >= character.airborne_punch_string_max &&
                    character.airborne_kick_string_curr >= character.airborne_kick_string_max &&
                    character.directional_attacks_mask_curr & 0b1111u32 > 0
                ;

                let combat_item = item.item_id == 4 || 
                item.item_id == 5 || item.item_id == 6 || 
                item.item_id == 7 || item.item_id == 8 || 
                item.item_id == 10 || item.item_id == 11 ||
                item.item_id == 12 || item.item_id == 15;

                let already_mastered = combat_item && all_attacks_mastered;

            println!("max_punches_already {} 
            max_kicks_already {} 
            max_air_punches_already {} 
            max_air_kicks_already {} 
            special_mask {} 
            all_attacks_mastered {}", 
            character.punch_string_curr >= character.punch_string_max,
            character.kick_string_curr >= character.kick_string_max,
            character.airborne_punch_string_curr >= character.airborne_punch_string_max, 
            character.airborne_kick_string_curr >= character.airborne_kick_string_max, 
            character.directional_attacks_mask_curr, 
            all_attacks_mastered);
            
            !(max_punches_already || max_kicks_already || max_air_punches_already || max_air_kicks_already || 
                launch_already || dropper_already || dashing_already || crash_already ||
                already_mastered)
        });
        table.acc = table.items.iter().map(|i|{i.rarity}).sum();
        
    }
}
