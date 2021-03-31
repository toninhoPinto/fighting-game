use std::string::String;

use sdl2::pixels::Color;
use sdl2::render::{WindowCanvas};
use sdl2::rect::{Point, Rect};

use parry2d::bounding_volume::AABB;

use crate::game_logic::characters::player::Player;
use crate::game_logic::projectile::Projectile;
use crate::game_logic::character_factory::CharacterAssets;

fn world_to_screen(rect: Rect, position: Point, screen_size: (u32, u32)) -> Rect {
    let (_, height) = screen_size;
    let mut inverted_pos = position;
    //to make world coordinates Y increase as we go up
    inverted_pos.y = -1 * inverted_pos.y;
    //first point is to make Y = 0 as the bottom of the screen
    //Second point it to make the bottom center of a rect as the position                                                         
    let screen_position = inverted_pos + Point::new(0, height as i32) + Point::new(0, -(rect.height() as i32) / 2);
    Rect::from_center(screen_position, rect.width(), rect.height())
}

fn debug_points(canvas: &mut WindowCanvas, screen_position: Point, rect_to_debug: Rect) {
    canvas.set_draw_color(Color::RGB(255, 100, 100));
    let debug_rect = Rect::new(screen_position.x as i32, screen_position.y as i32, 4, 4);

    canvas.draw_rect(debug_rect);
    canvas.fill_rect(debug_rect);

    canvas.draw_rect(rect_to_debug);
    canvas.set_draw_color(Color::RGBA(100, 50, 50, 50));
    canvas.fill_rect(rect_to_debug);


}

pub fn render<'a, 'b>(canvas: &mut WindowCanvas, color: Color,
              player1: &'b mut Player<'a>, p1_anims: &'a CharacterAssets,
              player2: &'b mut Player<'a>, p2_anims: &'a CharacterAssets,
              projectiles: &Vec<Projectile>, colliders: &Vec<AABB>, debug: bool)
    -> Result<(), String> {

    canvas.set_draw_color(color);
    canvas.clear();
    let screen_res = canvas.output_size()?;

    let screen_rect = world_to_screen(player1.character.sprite, player1.position, screen_res);
    let sprite = player1.character.sprite;
    let is_flipped = player1.flipped;
    let texture = player1.render(p1_anims);
    canvas.copy_ex(texture, sprite, screen_rect, 0.0, None, is_flipped, false)?;
    if debug {
        debug_points(canvas, screen_rect.center(), screen_rect);
        canvas.set_draw_color(color);
    }


    let screen_rect_2  = world_to_screen(player2.character.sprite, player2.position, screen_res);
    let sprite_2 = player2.character.sprite;
    let is_flipped_2 = player2.flipped;
    let texture_2 = player2.render(p2_anims);
    canvas.copy_ex(texture_2, sprite_2, screen_rect_2, 0.0, None, is_flipped_2, false)?;
    if debug {
        debug_points(canvas,screen_rect_2.center(), screen_rect_2);
        canvas.set_draw_color(color);
    }

    for projectile in projectiles.iter() {
        let screen_rect_2 = world_to_screen(projectile.sprite, projectile.position, screen_res);
        if projectile.player_owner == 1 {
            canvas.copy_ex(&p1_anims.projectile_animation.get(&projectile.animation_name).unwrap()[projectile.animation_index as usize], projectile.sprite, screen_rect_2, 0.0, None, projectile.flipped, false)?;
        } else if  projectile.player_owner == 2 {
            canvas.copy_ex(&p2_anims.projectile_animation.get(&projectile.animation_name).unwrap()[projectile.animation_index as usize], projectile.sprite, screen_rect_2, 0.0, None, projectile.flipped, false)?;
        }
        if debug {
            //debug_points(canvas,screen_rect_2.center(), screen_rect_2);
            //canvas.set_draw_color(color);
        }
    }

    for collider in colliders.iter() {
        let semi_transparent_green = Color::RGBA(50, 200, 100, 100);
        let collider_position = Point::new((collider.center().x + collider.half_extents().x)  as i32, (collider.center().y + collider.half_extents().y)  as i32);
        let screen_rect_2 = world_to_screen(Rect::new(0,0,collider.extents().x as u32, collider.extents().y as u32), collider_position, screen_res);

        canvas.draw_rect(screen_rect_2);
        canvas.set_draw_color(semi_transparent_green);
        canvas.fill_rect(screen_rect_2);

        if debug {
            //debug_points(canvas,screen_rect_2.center(), screen_rect);
            //canvas.set_draw_color(color);
        }
    }

    canvas.present();
    Ok(())
}