use std::fs;

use sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;

use super::{collider::ColliderAnimation, spriter_pro_collider_loader};


pub fn load_texture<'a>(
    tex_creator: &'a TextureCreator<WindowContext>,
    path: &'_ str,
) -> Texture<'a> {
    tex_creator.load_texture(path).unwrap()
}


pub fn load_anim_from_dir<'a>(
    tex_creator: &'a TextureCreator<WindowContext>,
    dir: &'_ str,
) -> Vec<(i32, Texture<'a>)> {
    let paths = fs::read_dir(dir).unwrap();

    let mut vec: Vec<(i32, Texture)> = Vec::new();
    let data: (Vec<i32>, ColliderAnimation);

    for entry in paths {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() && path.extension().unwrap() == "png" {
            vec.push((1,tex_creator.load_texture(path).unwrap()));
        }
    }

    vec
}


pub fn load_anim_and_data_from_dir<'a>(
    tex_creator: &'a TextureCreator<WindowContext>,
    dir: &'_ str,
) -> (Vec<(i32, Texture<'a>)>, Option<ColliderAnimation>) {
    let paths = fs::read_dir(dir).unwrap();

    let mut vec: Vec<(i32, Texture)> = Vec::new();
    let mut data: Option<(Vec<i32>, ColliderAnimation)> = None;

    let mut sprites_length = 0;
    for entry in paths {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() && path.extension().unwrap() == "png" {
            sprites_length += 3;
            vec.push((sprites_length,tex_creator.load_texture(path).unwrap()));
        } else if path.is_file() && path.extension().unwrap() == "scon" {
            data = Some(spriter_pro_collider_loader::load_animation_data(path))
        }
    }

    match data {
        Some(colliders) => {
            for i in 0..vec.len() {
                vec[i].0 = (colliders.0[i] / 16) as i32;
            }
        
            println!("{} {:?}",dir, vec.iter().map(|c| c.0).collect::<Vec<i32>>()    );
            (vec, Some(colliders.1))
        }
        _ => (vec, None)
    }
    
}
