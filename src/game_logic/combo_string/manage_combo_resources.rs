use sdl2::{pixels::Color, rect::Rect, render::{Texture, TextureCreator}, video::WindowContext};

use crate::{GameStateData, engine_types::simple_animator::{SimpleAnimator, init_combo_animation, init_compliment_animation}, rendering::renderer_ui::text_gen};

use super::ComboCounter;

pub struct Combo<'a> {
    pub combo_counter: ComboCounter,
    pub curr_combo_texture: Option<(u32, Texture<'a>, Texture<'a>)>,
    pub combo_rect: Rect,
    pub combo_animator: SimpleAnimator,
    pub combo_limit: Vec<u32>,
    pub combo_colors: Vec<Color>,
    pub combo_compliments: Vec<String>,
    
    pub compliment_text: Option<(Texture<'a>, Texture<'a>)>,
    pub compliment_animator: SimpleAnimator,
    pub compliment_rect: Rect,
    pub compliment_lifetime: f64,
    pub compliment_time_life: f64,

    pub combo_level: u32,
}

impl<'a> Combo<'a> {
    pub fn new() -> Self {
        let combo_rect = Rect::new(40, 200, 50, 50);
        let compliment_rect = Rect::new(60, 180, 250, 90);

        Self{
            combo_counter: ComboCounter::new(1.8f64),
            curr_combo_texture: None,
            combo_animator: init_combo_animation(combo_rect),
            combo_rect,
            combo_limit: vec![2, 8, 15, 25],
            combo_colors:  vec![Color::RGB(237, 222, 17), Color::RGB(237, 156, 17), Color::RGB(209, 10, 10), Color::RGB(107, 52, 235)],
            combo_compliments: vec!["Nice".to_string(), "Great".to_string(), "Amazing".to_string(), "Godlike".to_string()],
            compliment_text: None,
            compliment_animator: init_compliment_animation(compliment_rect),
            compliment_rect,
            compliment_lifetime: 0.85f64,
            compliment_time_life: 0f64,
            combo_level: 0,
        }

    }
}

pub fn update_and_manage<'a>(logic_timestep: f64, combo: &mut Combo<'a>, texture_creator: &'a TextureCreator<WindowContext>, game_state_data: &GameStateData) {

    combo.combo_counter.manage_combo(logic_timestep);
    let combo_val = combo.combo_counter.render();

    let combo_font = "combo_font".to_string();
    if let Some(combo_val) = combo_val {
        if let Some((val, _, _)) = &mut combo.curr_combo_texture {
            
            if *val != combo_val {

                let mut curr_combo_level = 0;
                for (i, &u32) in combo.combo_limit.iter().enumerate() {
                    if u32 > *val {
                        break;
                    } else {
                        curr_combo_level = i as u32;
                    }
                }

                combo.combo_animator.reset();

                if curr_combo_level != combo.combo_level {
                    combo.combo_animator.reset_full(&mut combo.combo_rect);
                    combo.combo_rect.set_width(combo.combo_rect.width()+10);
                    combo.combo_rect.set_height(combo.combo_rect.height()+10);
                    combo.combo_level = curr_combo_level;
                    
                    combo.combo_animator = init_combo_animation(combo.combo_rect);
                }

                combo.curr_combo_texture = Some((
                    combo_val, 
                    text_gen(combo_val.to_string(), texture_creator, game_state_data.general_assets.fonts.get(&combo_font).unwrap(), combo.combo_colors[combo.combo_level as usize]),
                    text_gen(combo_val.to_string(), texture_creator, game_state_data.general_assets.fonts.get(&combo_font).unwrap(), Color::BLACK),
                ));

                combo.combo_animator.play_once(9.0 + curr_combo_level as f64);
            }
        } else {
            combo.curr_combo_texture = Some((
                combo_val, 
                text_gen(combo_val.to_string(), texture_creator, game_state_data.general_assets.fonts.get(&combo_font).unwrap(), combo.combo_colors[0]),
                text_gen(combo_val.to_string(), texture_creator, game_state_data.general_assets.fonts.get(&combo_font).unwrap(), Color::BLACK),
            ));
            combo.combo_animator.reset();
            combo.combo_rect = Rect::new(20, 200, 50, 50);
            combo.combo_animator = init_combo_animation(combo.combo_rect);
            combo.combo_animator.play_once(9.0);
        }
    } else if !combo.curr_combo_texture.is_none() {
        combo.curr_combo_texture = None;
        combo.combo_animator.reset();
        combo.combo_rect = Rect::new(20, 200, 50, 50);
        combo.combo_animator = init_combo_animation(combo.combo_rect);
       

        combo.compliment_text =  Some(
            (
            text_gen(combo.combo_compliments[combo.combo_level as usize].clone(), texture_creator, game_state_data.general_assets.fonts.get(&combo_font).unwrap(), combo.combo_colors[combo.combo_level as usize]),
            text_gen(combo.combo_compliments[combo.combo_level as usize].clone(), texture_creator, game_state_data.general_assets.fonts.get(&combo_font).unwrap(), Color::BLACK)
            )
        );
        combo.compliment_animator.reset();
        combo.compliment_animator.play_once(6.5);
    }

    combo.combo_animator.update(&mut combo.combo_rect, logic_timestep);
    
    if combo.compliment_text.is_some() {
        combo.compliment_animator.update(&mut combo.compliment_rect, logic_timestep);
        
        if !combo.compliment_animator.is_playing {
            combo.compliment_time_life += logic_timestep;
        }

        if combo.compliment_lifetime < combo.compliment_time_life {
            combo.compliment_text = None;
            combo.compliment_time_life = 0f64;
        }
        
    }
    
}