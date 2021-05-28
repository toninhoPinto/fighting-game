use parry2d::na::Vector2;

use crate::{asset_management::asset_holders::EntityAnimations, ecs_system::enemy_components::Position, engine_types::animator::Animator, game_logic::{characters::player::{Player}, movement_controller::MovementController}, utils::math_sign::Sign};

pub fn walk_to_player(player: &Player, pos: &Position, controller: &mut MovementController, animator: &mut Animator)  {
    let dir_to_player = player.position - pos.0;

    controller.set_velocity(Vector2::new((dir_to_player.x as i8).sign() , 0), animator);
}