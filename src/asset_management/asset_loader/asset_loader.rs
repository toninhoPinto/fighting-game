use std::{collections::HashMap, fs};

use sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;

use crate::asset_management::{animation::{Animation, ColliderAnimation, Transformation}, cast_point::CastPoint, sprite_data::SpriteData};

use super::spriter_pro_collider_loader::{self, load_frame_data};


pub fn load_texture<'a>(
    tex_creator: &'a TextureCreator<WindowContext>,
    path: &'_ str,
) -> Texture<'a> {
    tex_creator.load_texture(path).unwrap()
}


pub fn load_textures_for_character<'a>(
    tex_creator: &'a TextureCreator<WindowContext>,
    dir: &'_ str,
) -> (HashMap<String, Texture<'a>>, HashMap<String, SpriteData>) {
    let mut textures = HashMap::new();
    let mut sprite_data = HashMap::new();

    look_for_textures(
        tex_creator,
        dir,
        &mut textures,
        &mut sprite_data,
    );
    (textures, sprite_data)
}

fn look_for_textures<'a>(
    tex_creator: &'a TextureCreator<WindowContext>,
    dir: &'_ str,
    textures_dictionary: &mut HashMap<String, Texture<'a>>,
    sprite_data_dictionary: &mut HashMap<String, SpriteData>
) {
    let paths = fs::read_dir(dir).unwrap();
    
    for entry in paths {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_dir() { 
            look_for_textures(tex_creator, path.to_str().unwrap(), textures_dictionary, sprite_data_dictionary);
        } else if path.is_file() && path.extension().unwrap() == "png" {
            let file_name = path.file_name().unwrap().to_str().unwrap().replace(".png", "").to_string();
            textures_dictionary.insert(file_name, tex_creator.load_texture(path).unwrap());
        } else if path.is_file() && path.extension().unwrap() == "scon" {
            let sprites_data: Vec<SpriteData> = load_frame_data(path);
            for data in sprites_data {
                sprite_data_dictionary.insert(data.sprite_name.clone(), data);
            }
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
    let mut data: Option<(Vec<i64>, ColliderAnimation, HashMap<i32, Transformation>, HashMap<i64, CastPoint>, i64)> = None;

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
    let mut sprite_alignments = HashMap::new();
    let mut length = sprites_length + 3;
    if let Some(colliders) = data {
        for i in 0..vec.len() {
            vec[i].0 = colliders.0[i];
        } 
        collider_animation = Some(colliders.1);
        sprite_alignments = colliders.2;
        points = colliders.3;
        length = colliders.4;
    }
    println!("animation name {:?} sprite_alignments {:?}", dir, sprite_alignments);
    Animation::new_with_data(vec, length, name.to_string(), None, points, sprite_alignments, collider_animation)
}
