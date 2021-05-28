use crate::{collision::collider_manager::ColliderManager, engine_types::animator::Animator, game_logic::{characters::Attack, movement_controller::MovementController}};


pub fn launch(attack: &Attack, collider_manager: &mut ColliderManager, mov: &mut MovementController, animator: &mut Animator) {
    mov.launch(attack, animator);
    collider_manager.init_colliders(animator);
}