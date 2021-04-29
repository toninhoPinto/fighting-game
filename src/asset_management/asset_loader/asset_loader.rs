use std::{collections::HashMap, fs};

use sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;

use crate::asset_management::{animation::{Animation, ColliderAnimation}, cast_point::CastPoint};

use super::spriter_pro_collider_loader;


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

pub fn load_anim_from_dir(dir: &str, name: &str) -> Animation {
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
    Animation::new(vec, sprites_length + 3, name.to_string(), None)
}

pub fn load_anim_and_data_from_dir(dir: &str, name: &str) -> Animation {
    let paths = fs::read_dir(dir).unwrap();

    let mut vec: Vec<(i64, String)> = Vec::new();
    let mut data: Option<(Vec<i64>, ColliderAnimation, HashMap<i64, CastPoint>, i64)> = None;
    println!("animation name {:?}", dir);
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

    let mut collider_animation = None;
    let mut points = HashMap::new();
    let mut length = sprites_length + 3;
    if let Some(colliders) = data {
        for i in 0..vec.len() {
            vec[i].0 = colliders.0[i];
        } 
        collider_animation = Some(colliders.1);
        points = colliders.2;
        length = colliders.3;
    }
    //println!("animation name {:?} points {:?}", dir, points);
    Animation::new_with_data(vec, length, name.to_string(), None, points, collider_animation)
}
