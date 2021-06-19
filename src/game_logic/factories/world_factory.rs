use sdl2::{render::TextureCreator, video::WindowContext};
use crate::asset_management::{asset_holders::OverworldAssets, asset_loader::{asset_loader::{self, load_texture, load_textures_for_character}, my_spritesheet_format::load_spritesheet}};

pub fn load_overworld_assets(texture_creator: &TextureCreator<WindowContext>) -> OverworldAssets {
    let spritesheet = asset_loader::load_texture(&texture_creator, "assets/overworld/spritesheet_default.png");

    let mapping = load_spritesheet("assets/overworld/spritesheet_mapping.json".to_string());
    
    OverworldAssets {
        spritesheet,
        src_rects: mapping,
        portraits: load_textures_for_character(texture_creator, "assets/portraits").0,
        backgrounds: vec![load_texture(texture_creator, "assets/stages/store.jpg")],
    }
}