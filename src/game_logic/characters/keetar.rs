use crate::game_logic::character_factory::CharacterAssets;
use crate::game_logic::projectile::Projectile;

use super::player::Player;
use parry2d::na::Vector2;
use sdl2::rect::Point;

//logic only module, no struct

pub fn spawn_note(
    player: &Player,
    opponent: &Player,
    assets: &CharacterAssets,
    projectiles: &mut Vec<Projectile>,
) {
    let player_current_animation = player.animator.current_animation.unwrap();
    let p1_curr_anim = player_current_animation.length;
    if (player.animator.sprite_shown as f32 + 0.35_f32) as usize >= p1_curr_anim as usize {
        //TODO currently it adds the projectile once at the end, but should add at specific key frames
        if player.is_attacking && assets.effects.contains_key(&player_current_animation.name) {
            let mut projectile =
                (*assets.effects.get(&player_current_animation.name).unwrap()).clone();
            
            projectile.position += Vector2::new(player.position.x, 0.0);
            projectile.direction.x = ((opponent.position.x - player.position.x) as i32).signum();
            projectile.flipped = player.dir_related_of_other > 0;
            projectile.player_owner = player.id;

            let target_pos = Vector2::new(
                opponent.position.x + (projectile.direction.x * 100) as f64,
                projectile.position.y,
            );
            projectile.target_position = Some(target_pos);
            projectiles.push(projectile);
        }
    }
}
