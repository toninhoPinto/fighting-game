use std::string::String;

use sdl2::{rect::{Point, Rect}, render::TextureQuery};
use sdl2::render::WindowCanvas;
use sdl2::{pixels::Color, render::Texture};

use crate::{game_logic::{character_factory::CharacterAssets, game::Game}, ui::ingame::end_match_ui::EndMatch};
use crate::{
    asset_management::collider::{Collider, ColliderType},
    game_logic::characters::player::Player,
    ui::ingame::{bar_ui::Bar, segmented_bar_ui::SegmentedBar},
};
use crate::{
    asset_management::{common_assets::CommonAssets, vfx::particle::Particle}
};

use super::camera::Camera;

fn world_to_screen(rect: Rect, position: Point, screen_size: (u32, u32), camera: &Camera) -> Rect {
    let (_, height) = screen_size;
    let mut inverted_pos = position;
    //make world coordinates Y increase as we go up
    //and make Y = 0 as the bottom of the screen
    inverted_pos.y = -inverted_pos.y + height as i32;
    inverted_pos.x -= camera.rect.x();

    //make the bottom center of a rect as the position
    let screen_position = inverted_pos - Point::new(0, (rect.height() as i32) / 2);
    Rect::from_center(screen_position, rect.width(), rect.height())
}

fn debug_points(canvas: &mut WindowCanvas, screen_position: Point, rect_to_debug: Rect) {
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
    camera: &mut Camera,
    stage: (&Texture, Rect),
    game: &mut Game,
    p1_assets: &CharacterAssets,
    p2_assets: &CharacterAssets,
    common_assets: &mut CommonAssets,
    hp_bars: &[Bar; 2],
    special_bars:  &[SegmentedBar; 2],
    //end_match_menu: &EndMatch,
    debug: bool,
) -> Result<(), String> {
    canvas.clear();

    canvas
        .copy(
            stage.0,
            camera.rect,
            Rect::new(0, 0, camera.rect.width(), camera.rect.height()),
        )
        .unwrap();

    let screen_res = canvas.output_size()?;

    let TextureQuery { width, height, .. } = common_assets.shadow.query();
    let shadow_rect = Rect::new(0, 0, width, (height as f64 * 1.5) as u32);
    let screen_rect = world_to_screen(shadow_rect, Point::new(game.player1.position.x as i32, -5), screen_res, camera);
    canvas.copy(&common_assets.shadow, shadow_rect, screen_rect)
        .unwrap();

    let screen_rect2 = world_to_screen(shadow_rect, Point::new(game.player2.position.x as i32, -5), screen_res, camera);
    canvas.copy(&common_assets.shadow, shadow_rect, screen_rect2)
        .unwrap();

    render_player(&mut game.player1, p1_assets, canvas, screen_res, camera, debug);
    render_player(&mut game.player2, p2_assets, canvas, screen_res, camera, debug);

    for projectile in game.projectiles.iter() {
        let screen_rect_2 =
            world_to_screen(projectile.sprite, Point::new(projectile.position.x as i32, projectile.position.y as i32) , screen_res, camera);

        let assets = if projectile.player_owner == 1 {p1_assets} else {p2_assets};
        canvas.copy_ex(
            projectile.render(assets),
            projectile.sprite,
            screen_rect_2,
            0.0,
            None,
            projectile.flipped,
            false,
        )?;

    }

    render_vfx(canvas, screen_res, camera, &mut game.hit_vfx, common_assets, debug);

    if debug {
        for i in 0..game.projectiles.len() {
            render_colliders(canvas, screen_res, camera, &mut game.projectiles[i].colliders);
        }
        render_colliders(canvas, screen_res, camera, &mut game.player1.colliders);
        render_colliders(canvas, screen_res, camera, &mut game.player2.colliders);
    }

    //Apparently sdl2 Rect doesnt like width of 0, it will make it width of 1, so i just stop it from rendering instead
    for i in 0..2{
        if hp_bars[i].fill_value > 0.0 {
            canvas.draw_rect(hp_bars[i].rect).unwrap();
            canvas.set_draw_color(hp_bars[i].color.unwrap());
            canvas.fill_rect(hp_bars[i].rect).unwrap();
        }
    
    }

    for i in 0..2 {
        if special_bars[i].curr_value > 0.0 {
            for j in 0..special_bars[i].render().len() {
                canvas.draw_rect(special_bars[i].rects[j]).unwrap();
                canvas.set_draw_color(special_bars[i].color.unwrap());
                canvas.fill_rect(special_bars[i].rects[j]).unwrap();
            }
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
    let screen_rect = world_to_screen(player.character.sprite, Point::new(player.position.x as i32, player.position.y as i32), screen_res, camera);
    let sprite = player.character.sprite;
    let is_flipped = player.flipped;
    let texture = player.render(assets);
    canvas
        .copy_ex(texture, sprite, screen_rect, 0.0, None, is_flipped, false)
        .unwrap();
    if debug {
        debug_points(canvas, screen_rect.center(), screen_rect);
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
                vfx.sprite.center().x,
                vfx.sprite.center().y - vfx.sprite.bottom() / 2,
            );
            let screen_rect = world_to_screen(rect_size, vfx_position, screen_res, camera);

            let (frame, texture_id) = &common_assets
                .hit_effect_animations
                .get_mut(&vfx.name)
                .unwrap()
                .sprites[vfx.animation_index as usize];

            canvas
                .copy_ex(common_assets.hit_effect_textures.get(texture_id).unwrap(), rect_size, screen_rect, 0.0, None, false, false)
                .unwrap();

            if debug {
                debug_points(canvas, screen_rect.center(), screen_rect);
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
            aabb.center().x as i32,
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
