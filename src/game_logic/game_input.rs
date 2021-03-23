use std::fmt;
use super::player::{Player, PlayerState};

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Debug)]
pub enum GameInputs {
    LightPunch,
    MediumPunch,
    HeavyPunch,
    LightKick,
    MediumKick,
    HeavyKick,
    Horizontal (i32),
    Vertical (i32)
}

impl fmt::Display for GameInputs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn apply_game_inputs(player: &mut Player, input: GameInputs){
    match input {
        GameInputs::Vertical(v) => {
            if v < 0 {
                println!("Jump")
            } else if v > 0 {
                println!("Crouching");
                player.state = PlayerState::Crouching;
                player.animation_index = 0.0;
                //player.current_animation = player1.animations.get("crouch").unwrap();
            } else {
                println!("Standing");
                player.state = PlayerState::Standing;
            }
        },
        GameInputs::Horizontal(h) => {
            player.direction = h;
        },
        GameInputs::LightPunch => {
            println!("Light Punch");
            player.isAttacking = true;
            player.animation_index = 0.0;
            player.current_animation = player.animations.get("light_punch").unwrap();
        },
        GameInputs::MediumPunch => { () },
        GameInputs::HeavyPunch => { () },
        GameInputs::LightKick => {
            println!("Light Kick")
        },
        GameInputs::MediumKick => { () },
        GameInputs::HeavyKick => { () },
        _ => { () }
    }
}
