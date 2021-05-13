use crate::game_logic::{characters::Attack, movement_controller::MovementController};


pub fn launch(attack: &Attack, mov: &mut MovementController) {
    mov.launch(attack);
}