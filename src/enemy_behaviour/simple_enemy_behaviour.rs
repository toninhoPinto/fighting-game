use parry2d::na::Vector2;

use crate::{collision::collider_manager::ColliderManager, ecs_system::{enemy_components::{Behaviour, Position}, enemy_systems::attack}, engine_types::animator::Animator, game_logic::{characters::player::{EntityState, Player}, movement_controller::MovementController}, utils::math_sign::Sign};

pub struct BasicEnemy {
    delay_between_punches: f64,
    time_accumulator: f64,
}

impl BasicEnemy {
    pub fn new() -> Self{
        Self {
            delay_between_punches: 1f64,
            time_accumulator: 0f64,
        }
    }
}

impl Behaviour for BasicEnemy {
    fn act(&mut self, player: &Player, pos: &Position, controller: &mut MovementController, animator: &mut Animator, collision_manager: &mut ColliderManager, delta_time: f64)  {
        let dir_to_player = player.position - pos.0;

        let hurt = controller.state == EntityState::Hurt || controller.state == EntityState::Knocked || controller.state == EntityState::Dropped || controller.state == EntityState::Dead;
        let recovering = controller.state == EntityState::KnockedLanding || controller.state == EntityState::DroppedLanding;
        if !controller.is_airborne && !hurt && !recovering {
            if (player.position.x - pos.0.x).abs() > 180f64 {
                controller.set_velocity(Vector2::new((dir_to_player.x as i8).sign() , 0), animator);
            } else {
                self.time_accumulator += delta_time;
                controller.set_velocity(Vector2::new(0 , 0), animator);
                if self.time_accumulator > self.delay_between_punches {
                    attack(controller, animator, collision_manager, "attack".to_string());
                    self.time_accumulator = 0f64;
                }
                
            }
        }
    }
}