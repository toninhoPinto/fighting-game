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

//TODO This mess needs a refactor
pub fn apply_game_inputs(player: &mut Player, input: GameInputs, last_inputs: &mut VecDeque<GameInputs>){
    match input {
        GameInputs::Vertical(v) => {
            if v < 0 {
                println!("Jump");
                player.last_directional_input_v = Some(GameInputs::UP);
            } else if v > 0 {
                println!("Crouching");
                player.state = PlayerState::Crouching;
                player.last_directional_input_v = Some(GameInputs::DOWN);
                //player.current_animation = player1.animations.get("crouch").unwrap();
            } else {
                println!("Standing");
                player.state = PlayerState::Standing;
                player.last_directional_input_v = None;
            }
            println!("pre-merge V {:?} {:?} {:?}", player.last_directional_input_h, player.last_directional_input_v, last_inputs);
            merge_last_horizontal_and_vertical_inputs(player, last_inputs);
        },
        GameInputs::Horizontal(h) => {
            player.direction = h;
            if h != 0 {
                if player.direction * player.dir_related_of_other > 0 {
                    player.last_directional_input_h = Some(GameInputs::FWD);
                } else {
                    player.last_directional_input_h = Some(GameInputs::BACK);
                }
            } else {
                player.last_directional_input_h = None;
            }
            println!("pre-merge H {:?} {:?} {:?}", player.last_directional_input_h, player.last_directional_input_v, last_inputs);
            merge_last_horizontal_and_vertical_inputs(player, last_inputs);
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
    println!("{:?} {:?} {:?}", player.last_directional_input_h, player.last_directional_input_v, last_inputs);
}

fn merge_last_horizontal_and_vertical_inputs(player: &mut Player, last_inputs: &mut VecDeque<GameInputs>){
    match (player.last_directional_input_h, player.last_directional_input_v)  {
        (Some(GameInputs::FWD), Some(GameInputs::DOWN)) => {
            player.last_directional_input = Some(GameInputs::FwdDOWN);
            record_input(last_inputs, GameInputs::FwdDOWN);
        },
        (Some(GameInputs::BACK), Some(GameInputs::DOWN)) => {
            player.last_directional_input = Some(GameInputs::BackDOWN);
            record_input(last_inputs, GameInputs::BackDOWN);
        },
        (Some(GameInputs::FWD), Some(GameInputs::UP)) => {
            player.last_directional_input = Some(GameInputs::FwdUP);
            record_input(last_inputs, GameInputs::FwdUP);
        },
        (Some(GameInputs::BACK), Some(GameInputs::UP)) => {
            player.last_directional_input = Some(GameInputs::BackUP);
            record_input(last_inputs, GameInputs::BackUP);
        },
        (None, a) if a.is_none()=> { },
        (a, None) => {
            player.last_directional_input = a;
            record_input(last_inputs, a.unwrap());
        },
        (None, a) => {
            player.last_directional_input = a;
            record_input(last_inputs, a.unwrap());
        },
        _ => {}
    }
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
    println!("{:?} {:?}", last_inputs, player.input_combination_anims);
    let mut l = 0;
    let mut ability_name: &str = "";
    'search_combo: for possible_combo in player.input_combination_anims.iter() {
        for n in 0..last_inputs.len() {
            l = 0;
            for d in n..last_inputs.len() {
                let (moves, name) = possible_combo;
                if last_inputs[d] == moves[l] {
                    l+= 1;
                } else {
                    break;
                }

                if l == moves.len() {
                    ability_name = name;
                    last_inputs.clear();
                    break 'search_combo;
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

