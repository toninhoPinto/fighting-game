use crate::asset_management::asset_loader::load_tiled_map::load_level;

use super::Level;


pub fn generate_levels(seed: u32) -> Vec<Level> {
    let map = load_level("assets/level/level1.tmx".to_string());
    let level = Level::new(map);
    vec![level]
}