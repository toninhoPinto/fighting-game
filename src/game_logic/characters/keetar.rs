use crate::game_logic::{character_factory::CharacterAssets, game::Game};

use parry2d::na::Vector2;

//logic only module, no struct

pub fn spawn_note(game: &mut Game<'_>, id: i32, assets: &CharacterAssets) {
    let (player,opponent) = if id == 1 { 
        (&*game.player1, &*game.player2) 
    } else { 
        (&*game.player2,  &*game.player1)
    };

    let player_current_animation = player.animator.current_animation.unwrap();
    if player.is_attacking && assets.projectiles.contains_key(&player_current_animation.name) {
        let mut projectile =
            (*assets.projectiles.get(&player_current_animation.name).unwrap()).clone();
        
        projectile.position += Vector2::new(player.position.x, 0.0);
        projectile.direction.x = ((opponent.position.x - player.position.x) as i32).signum();
        projectile.flipped = player.dir_related_of_other > 0;
        projectile.player_owner = player.id;

        let target_pos = Vector2::new(
            opponent.position.x + (projectile.direction.x * 100) as f64,
            projectile.position.y,
        );
        projectile.target_position = Some(target_pos);
        game.projectiles.push(projectile);
    }
}
