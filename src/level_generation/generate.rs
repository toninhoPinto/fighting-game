use std::collections::HashMap;

use rand::{Rng, prelude::SmallRng};
use tiled::Map;
use super::Level;

pub fn generate_levels(levels: &HashMap<i32, Map>, rng: &mut SmallRng) -> Vec<Level> {

    let n_possible_levels = levels.len() as u32;

    let random_number = rng.gen::<f64>();
    let n_levels = (random_number * n_possible_levels as f64) as u32 + 1;
    println!("{} {} {}", random_number, n_possible_levels, n_levels);

    let mut levels_spawned: Vec<Level> = Vec::new();

    for i in 0..n_levels {
        let map = levels.get(&0).unwrap();
        let start_pos = if let Some(last_level_spawned) =  levels_spawned.last() {
            last_level_spawned.start_x + (last_level_spawned.width * last_level_spawned.level_map.tile_width) as i32
        } else {
            0
        };
        println!("position {}", start_pos);
        let level = Level::new(map, start_pos);
        levels_spawned.push(level)
    }
    
    levels_spawned
}