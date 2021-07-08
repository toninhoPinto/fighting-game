use std::collections::HashMap;

use rand::{Rng, prelude::SmallRng};
use tiled::Map;
use super::Level;

pub fn generate_levels(levels: &HashMap<i32, Map>, rng: &mut SmallRng) -> Vec<Level> {

    let n_possible_levels = levels.len() as u32;

    let random_number = rng.gen::<f64>();
    let n_levels = (random_number * n_possible_levels as f64) as u32 + 1;

    let mut levels_spawned: Vec<Level> = Vec::new();

    for _ in 0..n_levels {

        let level_id = (rng.gen::<f64>() * n_possible_levels as f64) as i32;

        let map = levels.get(&level_id).unwrap();
        let start_pos = if let Some(last_level_spawned) =  levels_spawned.last() {
            last_level_spawned.start_x + (last_level_spawned.width * last_level_spawned.level_map.tile_width) as i32
        } else {
            0
        };

        let level = Level::new(map, start_pos);
        levels_spawned.push(level)
    }
    
    levels_spawned
}

pub fn get_levels(levels: &HashMap<i32, Map>, level_ids: &Vec<i32>) -> Vec<Level> {
    level_ids.iter().map(|id| {
        Level::new(levels.get(id).unwrap(), 0)
    }).collect::<Vec<Level>>()
}