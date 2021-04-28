use std::{collections::HashMap, fs};

use sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;

use super::{cast_point::CastPoint, collider::ColliderAnimation, spriter_pro_collider_loader};


pub fn load_texture<'a>(
    tex_creator: &'a TextureCreator<WindowContext>,
    path: &'_ str,
) -> Texture<'a> {
    tex_creator.load_texture(path).unwrap()
}


pub fn load_textures_for_character<'a>(
    tex_creator: &'a TextureCreator<WindowContext>,
    dir: &'_ str,
) -> HashMap<String, Texture<'a>> {
    let mut textures = HashMap::new();

    look_for_textures(
        tex_creator,
        dir,
        &mut textures,
    );
    textures
}

fn look_for_textures<'a>(
    tex_creator: &'a TextureCreator<WindowContext>,
    dir: &'_ str,
    textures_dictionary: &mut HashMap<String, Texture<'a>>
) {
    let paths = fs::read_dir(dir).unwrap();

    for entry in paths {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_dir() { 
            look_for_textures(tex_creator, path.to_str().unwrap(), textures_dictionary);
        } else if path.is_file() && path.extension().unwrap() == "png" {
            let file_name = path.file_name().unwrap().to_str().unwrap().replace(".png", "").to_string();
            textures_dictionary.insert(file_name, tex_creator.load_texture(path).unwrap());
        }
    }
}

pub fn load_anim_from_dir(dir: &str) -> Vec<(i64, String)> {
    let paths = fs::read_dir(dir).unwrap();

    let mut vec: Vec<(i64, String)> = Vec::new();

    let mut sprites_length = 0;
    for entry in paths {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() && path.extension().unwrap() == "png" {
            let file_name = path.file_name().unwrap().to_str().unwrap().replace(".png", "").to_string();
            sprites_length += 3;
            vec.push((sprites_length, file_name));
        }
    }
    vec
}

pub fn load_anim_and_data_from_dir(dir: &str) -> (Vec<(i64, String)>, Option<ColliderAnimation>) {
    let paths = fs::read_dir(dir).unwrap();

    let mut vec: Vec<(i64, String)> = Vec::new();
    let mut data: Option<(Vec<i64>, ColliderAnimation)> = None;

    let mut sprites_length = 0;
    for entry in paths {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() && path.extension().unwrap() == "png" {
            let file_name = path.file_name().unwrap().to_str().unwrap().replace(".png", "").to_string();
            sprites_length += 3;
            vec.push((sprites_length, file_name));
        } else if path.is_file() && path.extension().unwrap() == "scon" {
            data = Some(spriter_pro_collider_loader::load_animation_data(path))
        }
    }

    match data {
        Some(colliders) => {
            for i in 0..vec.len() {
                vec[i].0 = colliders.0[i] / 16;
            }
            (vec, Some(colliders.1))
        }
        _ => (vec, None)
    }
}
