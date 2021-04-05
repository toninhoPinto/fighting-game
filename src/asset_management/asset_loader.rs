use std::fs;

use sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;

use super::{collider::ColliderAnimation, spriter_pro_collider_loader};

pub fn load_hitboxes(file: std::string::String) -> ColliderAnimation {
    let colliders = spriter_pro_collider_loader::load_hitboxes(file);

    ColliderAnimation {
        colliders: colliders.0,
        pos_animations: colliders.1,
    }
}

pub fn load_anim_from_dir<'a>(
    tex_creator: &'a TextureCreator<WindowContext>,
    dir: &'_ str,
) -> Vec<Texture<'a>> {
    let paths = fs::read_dir(dir).unwrap();

    let mut vec: Vec<Texture> = Vec::new();

    for entry in paths {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() && path.extension().unwrap() == "png" {
            vec.push(tex_creator.load_texture(path).unwrap());
        }
    }
    vec
}
