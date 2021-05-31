use parry2d::na::Vector2;

use crate::{collision::collider_manager::ColliderManager, ecs_system::{enemy_components::Position, enemy_systems::attack}, engine_types::animator::Animator, game_logic::{characters::player::{EntityState, Player}, movement_controller::MovementController}, utils::math_sign::Sign};

pub fn walk_to_player(player: &Player, pos: &Position, controller: &mut MovementController, animator: &mut Animator, collision_manager: &mut ColliderManager)  {
    let dir_to_player = player.position - pos.0;

    let hurt = controller.state == EntityState::Hurt || controller.state == EntityState::Knocked || controller.state == EntityState::Dropped || controller.state == EntityState::Dead;
    let recovering = controller.state == EntityState::KnockedLanding || controller.state == EntityState::DroppedLanding;
    if !controller.is_airborne && !hurt && !recovering {
        if (player.position.x - pos.0.x).abs() > 150f64 {
            controller.set_velocity(Vector2::new((dir_to_player.x as i8).sign() , 0), animator);
        } else {
            controller.set_velocity(Vector2::new(0 , 0), animator);
            attack(controller, animator, collision_manager, "attack".to_string());
        }
    }
}