use std::{collections::HashMap, string::String};

use sdl2::{rect::{Point, Rect}, render::TextureQuery};
use sdl2::render::WindowCanvas;
use sdl2::{pixels::Color, render::Texture};

use crate::{asset_management::asset_holders::{EntityAssets, ItemAssets}, ecs_system::{enemy_systems::get_ground_pos_enemies}, engine_types::collider::{Collider, ColliderType}, game_logic::game::Game, level_generation::Level, scenes::overworld_scene::active_item_ui, ui::ingame::wrapping_list_ui::WrappingList};
use crate::{
    ui::ingame::{segmented_bar_ui::SegmentedBar},
};
use crate::{
    asset_management::{common_assets::CommonAssets, vfx::particle::Particle}
};

use super::camera::Camera;

pub fn pos_world_to_screen(position: Point, screen_size: (u32, u32), camera: Option<&Camera>) -> Point {
    let (_, height) = screen_size;
    let mut inverted_pos = position;
    //make world coordinates Y increase as we go up
    //and make Y = 0 as the bottom of the screen
    inverted_pos.y = -inverted_pos.y + height as i32;
    if let Some(camera) = camera {
        inverted_pos.x -= camera.get_camera().x();
    }
     //make camera as its own little space coordinates

    inverted_pos
}

pub fn world_to_screen(rect: Rect, position: Point, screen_size: (u32, u32), camera: Option<&Camera>) -> Rect {
    let screen_position = pos_world_to_screen(position, screen_size, camera);
    Rect::new(screen_position.x, screen_position.y - rect.height() as i32, rect.width(), rect.height())
}

pub fn world_to_screen_rect(rect: Rect, camera: Option<&Camera>) -> Rect {
    let mut inverted_pos = Point::new(rect.x(), rect.y());
    if let Some(camera) = camera {
        inverted_pos.x -= camera.get_camera().x();
    }

    let screen_position = inverted_pos;
    Rect::new(screen_position.x, screen_position.y - rect.height() as i32, rect.width(), rect.height())
}


fn debug_point(canvas: &mut WindowCanvas, screen_position: Point, color: Color) {
    canvas.set_draw_color(color);
    let debug_rect = Rect::new(screen_position.x as i32, screen_position.y as i32, 4, 4);

    canvas.draw_rect(debug_rect).unwrap();
    canvas.fill_rect(debug_rect).unwrap();
}

fn debug_rect(canvas: &mut WindowCanvas, screen_position: Point, rect_to_debug: Rect) {
    canvas.set_draw_color(Color::RGB(255, 100, 100));
    let debug_rect = Rect::new(screen_position.x as i32, screen_position.y as i32, 4, 4);

    canvas.draw_rect(debug_rect).unwrap();
    canvas.fill_rect(debug_rect).unwrap();

    canvas.draw_rect(rect_to_debug).unwrap();
    canvas.set_draw_color(Color::RGBA(100, 50, 50, 50));
    canvas.fill_rect(rect_to_debug).unwrap();
}

pub fn render(
    canvas: &mut WindowCanvas,
    game: &mut Game,
    p1_assets: &EntityAssets,
    enemy_assets: &HashMap<&str, EntityAssets>,
    common_assets: &mut CommonAssets,
    item_assets: &ItemAssets,
    item_list: &WrappingList,
    hp_bars: &SegmentedBar,
    debug: bool,
) -> Result<(), String> {
    
    
    let screen_res = canvas.output_size()?;

    render_level(canvas, &game.levels, common_assets, screen_res, &game.camera);

    render_shadow(common_assets,
        canvas,
        Point::new(game.player.position.x as i32 , game.player.controller.ground_height as i32),  
        screen_res,
        &game.camera);

    let shadow_positions = get_ground_pos_enemies(&mut game.enemies);

    for pos in shadow_positions {
        render_shadow(common_assets,
            canvas,
            pos,  
            screen_res,
            &game.camera);
    }

    let mut entities_to_render = crate::ecs_system::enemy_systems::render_enemies(&mut game.enemies, enemy_assets);
    let data_to_render = game.player.render(p1_assets);
    entities_to_render.push(data_to_render);

    let mut items_on_ground = game.items_on_ground
        .iter_mut()
        .map(|item| {item.render(item_assets)})
        .collect::<Vec<(&Texture, Rect, Point, bool, i32)>>();

    entities_to_render.append(&mut items_on_ground);
    entities_to_render.sort_by(|a, b| b.4.cmp(&a.4));

    render_enemies(&entities_to_render, canvas, screen_res, &game.camera, debug);

    if debug {
        for collider_of_enemy in  game.enemies.collider_components.iter_mut() {
            if let Some(collider_of_enemy) = collider_of_enemy {
                render_colliders(canvas, screen_res, &game.camera, &mut collider_of_enemy.colliders);
            }
        }
    }

    for projectile in game.projectiles.iter() {
        let screen_rect =
            world_to_screen(projectile.sprite, Point::new(projectile.position.x as i32, projectile.position.y as i32) , screen_res, Some(&game.camera));

        let assets = p1_assets;
        canvas.copy_ex(
            projectile.render(assets),
            projectile.sprite,
            screen_rect,
            0.0,
            None,
            projectile.flipped,
            false,
        )?;

        if debug {
            debug_rect(canvas, screen_rect.center(), screen_rect);
        }

    }

    render_vfx(canvas, screen_res, &game.camera, &mut game.hit_vfx, common_assets, debug);

    if debug {
        for i in 0..game.projectiles.len() {
            render_colliders(canvas, screen_res, &game.camera, &mut game.projectiles[i].colliders);
        }
        render_colliders(canvas, screen_res, &game.camera, &mut game.player.collision_manager.colliders);
    }

    if let Some(active_item) = &game.player.active_item_key {
        let src_rect = item_assets.src_rects.get(active_item).unwrap();
        canvas.copy(&item_assets.spritesheet, src_rect.clone(), active_item_ui()).unwrap();
    }
    
    if hp_bars.curr_value > 0 {
        canvas.set_draw_color(hp_bars.color.unwrap());
        for hp_rect in hp_bars.render() {
            canvas.draw_rect(hp_rect).unwrap();
            canvas.fill_rect(hp_rect).unwrap();
        }
    }

    let item_list = item_list.render();
    let player = &game.player;
    if player.items.len() > 0 {
        for i in 0..player.items.len() {
            let src_rect = item_assets.src_rects.get(&player.items[i]).unwrap();
            let dst_rect = item_list[i];
            canvas.copy(&item_assets.spritesheet, src_rect.clone(), dst_rect).unwrap();
        }
    }

    Ok(())
}

fn render_level(canvas: &mut WindowCanvas, levels: &Vec<Level>, common_assets: &CommonAssets, screen_size: (u32, u32), camera: &Camera) {
    let camera_pos = camera.rect.x();
    let camera_width =  camera.rect.width();

    for level in levels.iter() {
        for tile in level.tiles.iter().enumerate() {
            let spritesheet = common_assets.level_tiles.get(&level.map.tilesets[0].name).unwrap();
            // println!("tile {:?}", tile);
            let src_rect = level.rect_from_index(tile.0 as u32);
            let mut dst_rect = world_to_screen_rect(*tile.1, Some(camera));

            canvas.copy(spritesheet, src_rect, dst_rect).unwrap();
        }


        for tag in level.map.object_groups[0].objects.iter() {
            //let tag_pos = Vector2::new(tag.x as f64, ((level.map.height * level.map.tile_height) as f32 - tag.y) as f64);
            let tag = world_to_screen_rect(Rect::new(tag.x as i32, tag.y as i32, 10, 10), Some(camera));
            canvas.draw_rect(tag).unwrap();
            canvas.set_draw_color(Color::BLUE);
            canvas.fill_rect(tag).unwrap();
        }
    }
}



fn render_shadow(common_assets: &mut CommonAssets,
    canvas: &mut WindowCanvas,
    point: Point,  
    screen_res: (u32, u32),
    camera: &Camera) {

    let TextureQuery { width, height, .. } = common_assets.shadow.query();
    let shadow_rect = Rect::new(0, 0, width, (height as f64 * 1.5) as u32);

    let shadow_height = point.y - (shadow_rect.height() / 2) as i32;

    let screen_rect = world_to_screen(shadow_rect, Point::new(
        point.x as i32 - (shadow_rect.width() / 2) as i32, 
        shadow_height), screen_res, Some(camera));
    
    canvas.copy(&common_assets.shadow, shadow_rect, screen_rect)
        .unwrap();
}

fn render_enemies<'a>(entities: &Vec<(&'a Texture<'a>, Rect, Point, bool, i32)>,  
    canvas: &mut WindowCanvas,
    screen_res: (u32, u32),
    camera: &Camera,
    debug: bool,) {
    
    for enemy in entities {
        let screen_rect = world_to_screen(enemy.1,enemy.2 , screen_res, Some(camera));

        canvas
            .copy_ex(enemy.0, enemy.1, screen_rect, 0.0, None, enemy.3, false)
            .unwrap();

        if debug {
            debug_rect(canvas, screen_rect.center(), screen_rect);
        }
    }
}

fn render_vfx(
    canvas: &mut WindowCanvas,
    screen_res: (u32, u32),
    camera: &Camera,
    hit_vfx: &mut Vec<Particle>,
    common_assets: &mut CommonAssets,
    debug: bool,
) {
    for vfx in hit_vfx.iter() {
        if vfx.active {
            let rect_size = Rect::new(0, 0, vfx.sprite.width(), vfx.sprite.height());
            let vfx_position = Point::new(
                vfx.sprite.center().x - vfx.sprite.width() as i32 / 2,
                vfx.sprite.center().y - vfx.sprite.height() as i32 / 2,
            );

            let screen_rect = world_to_screen(rect_size, vfx_position, screen_res, Some(camera));

            let (_frame, texture_id) = &common_assets
                .hit_effect_animations
                .get_mut(&vfx.name)
                .unwrap()
                .sprites[vfx.sprite_shown as usize];

            canvas
                .copy_ex(common_assets.hit_effect_textures.get(texture_id).unwrap(), rect_size, screen_rect, 0.0, None, vfx.flipped, false)
                .unwrap();

            if debug {
                debug_rect(canvas, screen_rect.center(), screen_rect);
            }
        }
    }
}

fn render_colliders(
    canvas: &mut WindowCanvas,
    screen_res: (u32, u32),
    camera: &Camera,
    colliders: &mut Vec<Collider>,
) {
    let semi_transparent_green = Color::RGBA(50, 200, 100, 100);
    let semi_transparent_red = Color::RGBA(200, 50, 100, 100);
    let semi_transparent_blue = Color::RGBA(100, 50, 200, 100);

    for collider in colliders.iter().rev() {
        if !collider.enabled {
            continue;
        }

        let aabb = collider.aabb;
        let collider_position = Point::new(
            aabb.center().x as i32 - aabb.half_extents().x as i32,
            aabb.center().y as i32 - aabb.half_extents().y as i32,
        );
        let collider_rect_size = Rect::new(0, 0, aabb.extents().x as u32, aabb.extents().y as u32);
        let screen_rect_2 =
            world_to_screen(collider_rect_size, collider_position, screen_res, Some(camera));

        canvas.draw_rect(screen_rect_2).unwrap();
        if collider.collider_type == ColliderType::Hurtbox {
            canvas.set_draw_color(semi_transparent_green);
        } else if collider.collider_type == ColliderType::Pushbox {
            canvas.set_draw_color(semi_transparent_blue);
        } else {
            canvas.set_draw_color(semi_transparent_red);
        }
        canvas.fill_rect(screen_rect_2).unwrap();
    }
}
