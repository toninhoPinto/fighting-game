use std::fmt;
use super::player::{Player, PlayerState};
use std::collections::VecDeque;

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Debug)]
pub enum GameInputs {
    LightPunch,
    MediumPunch,
    HeavyPunch,
    LightKick,
    MediumKick,
    HeavyKick,
    Horizontal (i32),
    Vertical (i32),
    FWD,
    FwdDOWN,
    FwdUP,
    BACK,
    BackDOWN,
    BackUP,
    UP,
    DOWN
}

impl fmt::Display for GameInputs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn apply_game_inputs(player: &mut Player, input: GameInputs, last_inputs: &mut VecDeque<GameInputs>){
    println!("{:?}", player.last_directional_input);
    match input {
        GameInputs::Vertical(v) => {
            if v < 0 {
                println!("Jump");
                player.last_directional_input = Some(GameInputs::UP);
                record_input(last_inputs, GameInputs::UP);
            } else if v > 0 {
                println!("Crouching");
                player.state = PlayerState::Crouching;
                player.animation_index = 0.0;
                player.last_directional_input = Some(GameInputs::DOWN);
                record_input(last_inputs, GameInputs::DOWN);
                //player.current_animation = player1.animations.get("crouch").unwrap();
            } else {
                println!("Standing");
                player.state = PlayerState::Standing;

                match player.last_directional_input {
                    Some(GameInputs::UP) => player.last_directional_input = None,
                    Some(GameInputs::DOWN) => player.last_directional_input = None,
                     _ => {}
                }
            }
        },
        GameInputs::Horizontal(h) => {
            player.direction = h;
            println!("walk {}", h);
            if h != 0 {
                if player.direction * player.dir_related_of_other > 0 {
                    record_input(last_inputs, GameInputs::FWD);
                    player.last_directional_input = Some(GameInputs::FWD);
                } else {
                    record_input(last_inputs, GameInputs::BACK);
                    player.last_directional_input = Some(GameInputs::BACK);
                }
            } else {
                println!("Horizontal None");
                match player.last_directional_input {
                    Some(GameInputs::FWD) => player.last_directional_input = None,
                    Some(GameInputs::BACK) => player.last_directional_input = None,
                    _ => {}
                }
            }
        },
        GameInputs::LightPunch => {

            record_input(last_inputs, GameInputs::LightPunch);

            //TODO add input buffering both on button down and button up
            //but only buffer on button up IF already attacking, dont normal attack

            let special_attack = check_for_history_string_inputs(last_inputs, player);
            if special_attack != "" {
                player_attack(player, special_attack);
            } else { //check for directional inputs and if nothing then normal light punch
                let directional_attack = check_for_last_directional_inputs_directional_attacks(GameInputs::LightPunch, &player);
                if directional_attack != "" {
                    player_attack(player, directional_attack);
                } else {
                    player_attack(player, "light_punch");
                }

            }
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
    //println!("{:?}", last_inputs);
}


fn check_for_last_directional_inputs_directional_attacks<'a>(current_input: GameInputs , player: &Player<'a>) -> &'a str {
    let mut ability_name: &str = "";
    'search_directionals: for possible_combo in player.directional_variation_anims.iter() {
        let (moves, name) = possible_combo;

        match player.last_directional_input {
            Some(v) => {  if moves[0] == player.last_directional_input.unwrap() && moves[1] == current_input {
                                ability_name = name;
                                break 'search_directionals;
                            }
                        },
            None => {},
        }
    }
    ability_name
}


fn check_for_history_string_inputs<'a>(last_inputs: &mut VecDeque<GameInputs>, player: &Player<'a>) -> &'a str {
    //iterate over last inputs starting from the end
    //check of matches against each of the player.input_combination_anims
    //if no match
    // iterate over last inputs starting from the end -1
    //etc
    //if find match, play animation and remove that input from array
    let mut l = 0;
    let mut ability_name: &str = "";
    'search_combo: for possible_combo in player.input_combination_anims.iter() {
        for n in 0..last_inputs.len() {
            for d in 0..(last_inputs.len()-n-1) {
                let (moves, name) = possible_combo;
                if l == moves.len() {
                    ability_name = name;

                    last_inputs.clear();
                    break 'search_combo;
                }
                if last_inputs[d] == moves[l] {
                    l+= 1;
                }
            }
        }
    }

    ability_name
}

fn record_input(last_inputs: &mut VecDeque<GameInputs>, input: GameInputs){
    last_inputs.push_back(input);
    if last_inputs.len() > 5 {
        last_inputs.pop_front();
    }
}

fn player_attack(player: &mut Player, attack_animation: &str) {
    if !player.isAttacking {
        player.isAttacking = true;
        player.animation_index = 0.0;
        player.current_animation = player.animations.get(attack_animation).unwrap();
    }
}

