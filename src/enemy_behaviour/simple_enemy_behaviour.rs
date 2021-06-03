use crate::{ecs_system::{enemy_components::{Behaviour}}, game_logic::inputs::game_inputs::GameAction};

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
    fn act(&mut self, dt: f64) -> Option<GameAction> {
        self.time_accumulator += dt;
        if self.time_accumulator > self.delay_between_punches {
            self.time_accumulator = 0f64;
            return Some(GameAction::Punch);
        }
        return None;
    }
}