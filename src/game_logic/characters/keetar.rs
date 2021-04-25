use crate::game_logic::{character_factory::CharacterAnimations, game::Game, projectile::Projectile};

use parry2d::na::Vector2;

//logic only module, no struct

pub fn spawn_light_note(game: &mut Game, id: i32, assets: &CharacterAnimations) {
    spawn_note(game, id, Projectile::new(0, Vector2::new(120.0, 5.0)), assets);
}

pub fn spawn_medium_note(game: &mut Game, id: i32, assets: &CharacterAnimations) {
    spawn_note(game, id, Projectile::new(0, Vector2::new(120.0, 105.0)), assets);
}

pub fn spawn_heavy_note(game: &mut Game, id: i32, assets: &CharacterAnimations) {
    spawn_note(game, id,  Projectile::new(0, Vector2::new(120.0, 205.0)), assets);
}


fn spawn_note(game: &mut Game, id: i32, mut projectile: Projectile, assets: &CharacterAnimations) {
    let (player,opponent) = if id == 1 { 
        (&game.player1, &game.player2) 
    } else { 
        (&game.player2,  &game.player1)
    };

    if player.is_attacking {
        projectile.position += Vector2::new(player.position.x, 0.0);
        projectile.direction.x = player.dir_related_of_other.signum();
        projectile.flipped = player.dir_related_of_other > 0;
        projectile.player_owner = player.id;
        projectile.damage = 10;
        projectile.speed = 15;

        let target_pos = Vector2::new(
            opponent.position.x + (projectile.direction.x * 100) as f64,
            projectile.position.y,
        );

        projectile.target_position = Some(target_pos);
        projectile.init(assets.projectile_animation.get("note").unwrap().clone(), assets.projectile_collider_animations.get("note").unwrap().colliders.clone());
        game.projectiles.push(projectile);
    }
}
