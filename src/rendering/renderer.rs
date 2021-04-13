use std::string::String;

use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;

use crate::{asset_management::common_assets::CommonAssets, game_logic::character_factory::CharacterAssets};
use crate::game_logic::projectile::Projectile;
use crate::{
    asset_management::collider::{Collider, ColliderType},
    game_logic::characters::player::Player,
    ui::ingame::{bar_ui::Bar, segmented_bar_ui::SegmentedBar},
};

fn world_to_screen(rect: Rect, position: Point, screen_size: (u32, u32)) -> Rect {
    let (_, height) = screen_size;
    let mut inverted_pos = position;
    //to make world coordinates Y increase as we go up
    inverted_pos.y *= -1; 
    //first point is to make Y = 0 as the bottom of the screen
    //Second point it to make the bottom center of a rect as the position
    let screen_position =
        inverted_pos + Point::new(0, height as i32) + Point::new(0, -(rect.height() as i32) / 2);
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

pub fn render<'a, 'b>(
    canvas: &mut WindowCanvas,
    color: Color,
    player1: &'b mut Player<'a>,
    p1_assets: &'a CharacterAssets,
    player2: &'b mut Player<'a>,
    p2_assets: &'a CharacterAssets,
    projectiles: &[Projectile],
    hit_vfx: &mut Vec<(bool, Rect, String, i32)>,
    common_assets: &CommonAssets,
    p1_colliders: &mut Vec<Collider>,
    p2_colliders: &mut Vec<Collider>,
    bar_ui_1: &Bar,
    bar_ui_2: &Bar,
    bar_ui_3: &SegmentedBar,
    bar_ui_4: &SegmentedBar,
    debug: bool,
) -> Result<(), String> {
    canvas.set_draw_color(color);
    canvas.clear();
    let screen_res = canvas.output_size()?;

    let screen_rect = world_to_screen(player1.character.sprite, player1.position, screen_res);
    let sprite = player1.character.sprite;
    let is_flipped = player1.flipped;
    let texture = player1.render();

    canvas.copy_ex(texture, sprite, screen_rect, 0.0, None, is_flipped, false)?;
    if debug {
        debug_points(canvas, screen_rect.center(), screen_rect);
        canvas.set_draw_color(color);
    }

    let screen_rect_2 = world_to_screen(player2.character.sprite, player2.position, screen_res);
    let sprite_2 = player2.character.sprite;
    let is_flipped_2 = player2.flipped;
    let texture_2 = player2.render();
    canvas.copy_ex(
        texture_2,
        sprite_2,
        screen_rect_2,
        0.0,
        None,
        is_flipped_2,
        false,
    )?;
    if debug {
        debug_points(canvas, screen_rect_2.center(), screen_rect_2);
        canvas.set_draw_color(color);
    }

    for projectile in projectiles.iter() {
        let screen_rect_2 = world_to_screen(projectile.sprite, projectile.position, screen_res);
        if projectile.player_owner == 1 {
            canvas.copy_ex(
                &p1_assets
                    .projectile_animation
                    .get(&projectile.animation_name)
                    .unwrap()[projectile.animation_index as usize],
                projectile.sprite,
                screen_rect_2,
                0.0,
                None,
                projectile.flipped,
                false,
            )?;
        } else if projectile.player_owner == 2 {
            canvas.copy_ex(
                &p2_assets
                    .projectile_animation
                    .get(&projectile.animation_name)
                    .unwrap()[projectile.animation_index as usize],
                projectile.sprite,
                screen_rect_2,
                0.0,
                None,
                projectile.flipped,
                false,
            )?;
        }
    }

    render_vfx(canvas,
        screen_res,
        hit_vfx,
        common_assets,
        debug);

    if debug {
        render_colliders(canvas, screen_res, p1_colliders);
        render_colliders(canvas, screen_res, p2_colliders);
    }

    //Apparently sdl2 Rect doesnt like width of 0, it will make it width of 1, so i just stop it from rendering instead
    if bar_ui_1.fill_value > 0.0 {
        canvas.draw_rect(bar_ui_1.rect).unwrap();
        canvas.set_draw_color(bar_ui_1.color.unwrap());
        canvas.fill_rect(bar_ui_1.rect).unwrap();
    }

    if bar_ui_2.fill_value > 0.0 {
        canvas.draw_rect(bar_ui_2.rect).unwrap();
        canvas.set_draw_color(bar_ui_2.color.unwrap());
        canvas.fill_rect(bar_ui_2.rect).unwrap();
    }

    if bar_ui_3.curr_value > 0.0 {
        for i in 0..bar_ui_3.render().len() {
            canvas.draw_rect(bar_ui_3.rects[i]).unwrap();
            canvas.set_draw_color(bar_ui_3.color.unwrap());
            canvas.fill_rect(bar_ui_3.rects[i]).unwrap();
        }
    }

    if bar_ui_4.curr_value > 0.0 {
        for i in 0..bar_ui_4.render().len() {
            canvas.draw_rect(bar_ui_4.rects[i]).unwrap();
            canvas.set_draw_color(bar_ui_4.color.unwrap());
            canvas.fill_rect(bar_ui_4.rects[i]).unwrap();
        }
    }

    canvas.present();
    Ok(())
}

fn render_vfx(canvas: &mut WindowCanvas,
    screen_res: (u32, u32),
    hit_vfx: &mut Vec<(bool, Rect, String, i32)>,
    common_assets: &CommonAssets,
    debug: bool) {

    for vfx in hit_vfx.iter() {
        if vfx.0 {
            let rect_size = Rect::new(0, 0, vfx.1.width(), vfx.1.height());
            let vfx_position = Point::new(
                vfx.1.center().x,
                vfx.1.center().y - vfx.1.bottom() / 2,
            );
            let screen_rect = world_to_screen(rect_size, vfx_position, screen_res);
            canvas.copy_ex(
            &common_assets
                        .hit_effect_animations
                        .get(&vfx.2)
                        .unwrap()
                        .sprites[vfx.3 as usize],
                rect_size,
                screen_rect,
                0.0,
                None,
                false,
                false,
            ).unwrap();

            if debug {
                debug_points(canvas, screen_rect.center(), screen_rect);
            }
        }
    }
}

fn render_colliders(
    canvas: &mut WindowCanvas,
    screen_res: (u32, u32),
    colliders: &mut Vec<Collider>,
) {
    for collider in colliders.iter().rev() {
        let aabb = collider.aabb;
        let semi_transparent_green = Color::RGBA(50, 200, 100, 100);
        let semi_transparent_red = Color::RGBA(200, 50, 100, 150);
        let semi_transparent_blue = Color::RGBA(100, 50, 200, 150);
        let collider_position = Point::new(
            aabb.center().x as i32,
            aabb.center().y as i32 - aabb.half_extents().y as i32,
        );
        let collider_rect_size = Rect::new(0, 0, aabb.extents().x as u32, aabb.extents().y as u32);
        let screen_rect_2 = world_to_screen(collider_rect_size, collider_position, screen_res);

        canvas.draw_rect(screen_rect_2).unwrap();
        if collider.collider_type == ColliderType::Hurtbox {
            canvas.set_draw_color(semi_transparent_green);
        } else if collider.collider_type == ColliderType::Pushbox {
            canvas.set_draw_color(semi_transparent_blue);
        }else{
            canvas.set_draw_color(semi_transparent_red);
        }
        canvas.fill_rect(screen_rect_2).unwrap();
    }
}
