use std::string::String;

use sdl2::{rect::{Point, Rect}, render::TextureQuery};
use sdl2::render::WindowCanvas;
use sdl2::{pixels::Color, render::Texture};

use crate::game_logic::{character_factory::CharacterAssets, game::Game};
use crate::{
    asset_management::collider::{Collider, ColliderType},
    game_logic::characters::player::Player,
    ui::ingame::{bar_ui::Bar, segmented_bar_ui::SegmentedBar},
};
use crate::{
    asset_management::{common_assets::CommonAssets, vfx::particle::Particle}
};

use super::camera::Camera;

fn pos_world_to_screen(position: Point, screen_size: (u32, u32), camera: &Camera) -> Point {
    let (_, height) = screen_size;
    let mut inverted_pos = position;
    //make world coordinates Y increase as we go up
    //and make Y = 0 as the bottom of the screen
    inverted_pos.y = -inverted_pos.y + height as i32;
    inverted_pos.x -= camera.rect.x(); //make camera as its own little space coordinates

    inverted_pos
}

fn world_to_screen(rect: Rect, position: Point, screen_size: (u32, u32), camera: &Camera) -> Rect {
    let screen_position = pos_world_to_screen(position, screen_size, camera);
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
    stage: (&Texture, Rect),
    game: &mut Game,
    p1_assets: &CharacterAssets,
    common_assets: &mut CommonAssets,
    hp_bars: &Bar,
    special_bars:  &SegmentedBar,
    //end_match_menu: &EndMatch,
    debug: bool,
) -> Result<(), String> {
    canvas.clear();

    canvas
        .copy(
            stage.0,
            game.camera.rect,
            Rect::new(0, 0, game.camera.rect.width(), game.camera.rect.height()),
        )
        .unwrap();

    let screen_res = canvas.output_size()?;

    let TextureQuery { width, height, .. } = common_assets.shadow.query();
    let shadow_rect = Rect::new(0, 0, width, (height as f64 * 1.5) as u32);

    let shadow_height = game.player.ground_height as i32 - (shadow_rect.height() / 2) as i32;

    let screen_rect = world_to_screen(shadow_rect, Point::new(
        game.player.position.x as i32 - (shadow_rect.width() / 2) as i32, 
        shadow_height), screen_res, &game.camera);
    canvas.copy(&common_assets.shadow, shadow_rect, screen_rect)
        .unwrap();

    render_player(&mut game.player, p1_assets, canvas, screen_res, &game.camera, debug);

    for projectile in game.projectiles.iter() {
        let screen_rect =
            world_to_screen(projectile.sprite, Point::new(projectile.position.x as i32, projectile.position.y as i32) , screen_res, &game.camera);

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
        render_colliders(canvas, screen_res, &game.camera, &mut game.player.colliders);
    }

    //Apparently sdl2 Rect doesnt like width of 0, it will make it width of 1, so i just stop it from rendering instead
    if hp_bars.fill_value > 0.0 {
        canvas.draw_rect(hp_bars.rect).unwrap();
        canvas.set_draw_color(hp_bars.color.unwrap());
        canvas.fill_rect(hp_bars.rect).unwrap();
    }
    

    if special_bars.curr_value > 0.0 {
        for j in 0..special_bars.render().len() {
            canvas.draw_rect(special_bars.rects[j]).unwrap();
            canvas.set_draw_color(special_bars.color.unwrap());
            canvas.fill_rect(special_bars.rects[j]).unwrap();
        }
    }

    canvas.present();
    Ok(())
}

fn render_player(
    player: &mut Player,
    assets: &CharacterAssets,
    canvas: &mut WindowCanvas,
    screen_res: (u32, u32),
    camera: &Camera,
    debug: bool,
) {

    let is_flipped = player.facing_dir > 0;
    
    let sprite = player.character.sprite;

    let player_pos = player.position;
    let (texture, data) = player.render(assets);
    let pos = Point::new((player_pos.x - data.1.0) as i32, (player_pos.y - data.1.1 )as i32 );

    let screen_rect = world_to_screen(data.0,pos , screen_res, camera);

    canvas
        .copy_ex(texture, sprite, screen_rect, 0.0, None, is_flipped, false)
        .unwrap();
    if debug {
        let point = Point::new(player.position.x as i32, player.position.y as i32);
        debug_point(canvas, pos_world_to_screen(point,screen_res, camera), Color::RGB(50, 250, 255));
        debug_point(canvas, pos_world_to_screen(Point::new(player_pos.x as i32, player_pos.y as i32),screen_res, camera), Color::RGB(150, 255, 100));
        debug_rect(canvas, screen_rect.center(), screen_rect);
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

            let screen_rect = world_to_screen(rect_size, vfx_position, screen_res, camera);

            let (frame, texture_id) = &common_assets
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
    for collider in colliders.iter().rev() {
        if !collider.enabled {
            continue;
        }

        let aabb = collider.aabb;
        let semi_transparent_green = Color::RGBA(50, 200, 100, 100);
        let semi_transparent_red = Color::RGBA(200, 50, 100, 150);
        let semi_transparent_blue = Color::RGBA(100, 50, 200, 150);
        let collider_position = Point::new(
            aabb.center().x as i32 - aabb.half_extents().x as i32,
            aabb.center().y as i32 - aabb.half_extents().y as i32,
        );
        let collider_rect_size = Rect::new(0, 0, aabb.extents().x as u32, aabb.extents().y as u32);
        let screen_rect_2 =
            world_to_screen(collider_rect_size, collider_position, screen_res, camera);

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
