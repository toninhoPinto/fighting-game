use crate::{ecs_system::enemy_components::Position, game_logic::{characters::player::{Player}, factories::enemy_factory::EnemyAnimations, movement_controller::MovementController}};



pub fn walk_to_player(player: &Player, pos: &Position, controller: &mut MovementController, _enemy_animations: &EnemyAnimations)  {
    let dir_to_player = player.position - pos.0;

    controller.set_velocity_x(dir_to_player.x as i8);
}