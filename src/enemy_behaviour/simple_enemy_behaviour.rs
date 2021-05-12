use crate::{asset_management::asset_holders::EntityAnimations, ecs_system::enemy_components::Position, game_logic::{characters::player::{Player}, movement_controller::MovementController}};



pub fn walk_to_player(player: &Player, pos: &Position, controller: &mut MovementController, _enemy_animations: &EntityAnimations)  {
    let dir_to_player = player.position - pos.0;

    controller.set_velocity_x(dir_to_player.x as i8);
}