use crate::game_logic::character_factory::CharacterAssets;
use crate::game_logic::projectile::Projectile;
use crate::game_logic::game::Game;

//logic only module, no struct

pub fn spawn_projectile(world_state: &Game, player: &Player, character_assets: &CharacterAssets) {
    let mut projectile = (*character_assets.effects.get(event_name).unwrap()).clone();
    projectile.position = projectile.position.offset(player.position.x() + 200, 0); //TODO have animations with spawn points
    projectile.direction.x = player.dir_related_of_other;
    projectile.flipped = player.dir_related_of_other > 0;
    projectile.player_owner = player.id;

    let target_pos = Point::new(world_state.player2.position.x + (projectile.direction.x * 100), projectile.position.y);
    projectile.target_position = Some(target_pos);
    world_state.projectiles.push(projectile);
}