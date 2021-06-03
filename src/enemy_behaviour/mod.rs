use parry2d::na::Vector2;

use crate::{ecs_system::{enemy_components::{AIType, Behaviour, Health, Position}, enemy_manager::EnemyManager, enemy_systems::attack}, game_logic::{characters::{player::{EntityState, Player}}, inputs::game_inputs::GameAction, movement_controller::MovementController}, utils::math_sign::Sign};

pub mod simple_enemy_behaviour;


pub fn update_behaviour_enemies(enemy_manager: &mut EnemyManager, player: &mut Player, dt: f64) {
    let zip = enemy_manager.
        behaviour_components.iter_mut()
        .enumerate()
        .zip(enemy_manager.health_components.iter());

    let mut behaviour_entities = zip
    .filter_map(| ((i, behaviour), hp): 
    ((usize, &mut Option<Box<dyn Behaviour>>), &Option<Health>)| {
        if let Some(hp) = hp {
            if hp.0 > 0 {
                return Some((i, behaviour.as_mut()?))
            }
        }
        None
    })
    .collect::<Vec<(usize, &mut Box<dyn Behaviour>)>>();

    let entity_positions = &enemy_manager.positions_components;
    let entity_controllers = &mut enemy_manager.movement_controller_components;
    let entity_animators = &mut enemy_manager.animator_components;
    let entity_collision_managers = &mut enemy_manager.collider_components;
    let entity_ai_type = &mut enemy_manager.ai_type_components;

    behaviour_entities.iter_mut().for_each(|(index, behaviour) |{
        let pos = entity_positions[*index].as_ref().unwrap();
        let controller = entity_controllers[*index].as_mut().unwrap();
        let animator = entity_animators[*index].as_mut().unwrap();
        let collision_manager = entity_collision_managers[*index].as_mut().unwrap();
        let ai_type = entity_ai_type[*index].as_ref().unwrap();

        let target_pos = if *ai_type == AIType::Enemy { 
            Some(player.position)
        } else if *ai_type == AIType::Allied {

           let nr_enemies = entity_ai_type.iter().enumerate().zip(entity_positions.iter()).filter_map(|((usize, ai), pos)| { 
                if let Some(AIType::Enemy) = ai {
                    Some((usize, pos.as_ref()?))
                } else {
                    None
                }
            }).collect::<Vec<(usize, &Position)>>();

            if nr_enemies.len() >= 1 {
                let mut closest_enemy: Option<usize> = None;
                for (i, enemy_pos) in nr_enemies {
                    if closest_enemy.is_none() {
                        closest_enemy = Some(i);
                    }
                    let distance_to_curr = enemy_pos.0 - pos.0;
                    let distance_to_closest = entity_positions[closest_enemy.unwrap()].as_ref().unwrap().0 - pos.0;
                    
                    if distance_to_closest > distance_to_curr {
                        closest_enemy = Some(i);
                    }
                } 
                Some(entity_positions[closest_enemy.unwrap()].as_ref().unwrap().0)
            }
            else {
                None
            }
        } else {
            None
        };
        

        if let Some(target_pos) = target_pos {
            let dir_to_target = target_pos - pos.0;
            println!("dir {:?}", dir_to_target);

            let hurt = controller.state == EntityState::Hurt || controller.state == EntityState::Knocked || controller.state == EntityState::Dropped || controller.state == EntityState::Dead;
            let recovering = controller.state == EntityState::KnockedLanding || controller.state == EntityState::DroppedLanding;
            if !controller.is_airborne && !hurt && !recovering {
                if dir_to_target.x.abs() > 180f64 {
                    controller.set_velocity(Vector2::new((dir_to_target.x as i8).sign() , 0), animator);
                } else {
                    controller.set_velocity(Vector2::new(0 , 0), animator);
                    let action = behaviour.act(dt);
                    if let Some(action) = action {
                        match action {
                            GameAction::Punch => {
                                attack(controller, animator, collision_manager, "attack".to_string())
                            },
                            _ => {}
                        }
                    }
                }
            }
        } else {
            controller.set_velocity(Vector2::new(0 , 0), animator);
        }
        

    });
}


