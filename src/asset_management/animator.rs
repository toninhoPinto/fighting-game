use parry2d::na::Vector2;

use super::{animation::Animation};

#[derive(Clone)]
pub struct Animator {
    pub animation_index: f64,
    pub sprite_shown: i32,
    pub speed: f64,
    pub current_animation: Option<Animation>,
    pub is_playing: bool,
    pub is_finished: bool,
    pub play_once: bool,
    pub rewind: bool,
}

impl Animator {
    pub fn new() -> Self {
        Self {
            animation_index: 0.0,
            sprite_shown: 0,
            speed: 1.0,
            current_animation: None,
            is_playing: false,
            is_finished: false,
            play_once: false,
            rewind: false,
        }
    }

    pub fn play(&mut self, new_animation: Animation, speed: f64, play_rewind: bool) {
        self.play_animation(new_animation, speed, play_rewind,  false, false);
    }

    pub fn play_once(&mut self, new_animation: Animation, speed: f64, play_rewind: bool) {
        self.play_animation(new_animation, speed, play_rewind,  true, false);
    }

    pub fn play_animation(&mut self, new_animation: Animation, speed: f64, play_rewind: bool, play_once: bool, interrupt_self: bool){
        if interrupt_self || self.current_animation.is_none() || (self.current_animation.as_ref().unwrap().name != new_animation.name)
        {
            if !play_rewind {
                self.animation_index = 0.0;
                self.sprite_shown = 0; 
            } else {
                self.animation_index = new_animation.length as f64;
                self.sprite_shown = new_animation.sprites.len() as i32 - 1;
            }
            self.current_animation = Some(new_animation);
            self.play_once = play_once;
            self.is_playing = true;
            self.is_finished = false;
            self.rewind = play_rewind;
            self.speed = speed;
        }
    }

    fn finished_animation(&mut self) {
        self.is_playing = false;
        self.is_finished = true;
    }

    pub fn update(&mut self) {
        let playing_animation = self.current_animation.as_ref().unwrap();
        let n_sprites = playing_animation.sprites.len() as i32;

        if self.is_playing {

            if !self.rewind {
                self.animation_index += self.speed;

                let time_over_animation_length = self.animation_index >= playing_animation.length as f64;
                let time_to_switch_to_next_sprite = self.sprite_shown < (playing_animation.sprites.len() - 1) as i32 
                && (playing_animation.sprites[self.sprite_shown as usize + 1].0 as f64) <= self.animation_index;

                if time_over_animation_length || time_to_switch_to_next_sprite {
                    self.sprite_shown += 1;
                }
            } else {
                if !self.play_once {
                    if self.animation_index - self.speed < 0.0 {
                        self.animation_index = playing_animation.length as f64;
                        self.sprite_shown = n_sprites - 1;
                    } else {
                        self.animation_index = self.animation_index - self.speed;
                    }
                } else {
                    self.animation_index = (self.animation_index - self.speed).max(0.0);
                }

                let looped_sprite= std::cmp::max(0, self.sprite_shown - 1);

                if playing_animation.sprites[looped_sprite as usize].0 as f64 >= self.animation_index {
                    self.sprite_shown = looped_sprite;
                }
            }

            if self.play_once {
                if !self.rewind {
                    if self.animation_index >= playing_animation.length as f64 {
                        self.animation_index = playing_animation.length as f64;
                        self.sprite_shown = playing_animation.sprites.len() as i32 - 1;
                        self.finished_animation();
                    }
                } else {
                    if self.animation_index <= 0.5 {
                        self.animation_index = 0.0;
                        self.sprite_shown = 0;
                        self.finished_animation();
                    }
                }
            } else if !self.rewind {
                self.animation_index = self.animation_index.abs() % playing_animation.length as f64;
                self.sprite_shown = self.sprite_shown.abs() % playing_animation.sprites.len() as i32;
            }
        }
    }
  
    pub fn render(&self) -> String {
        self.current_animation.as_ref().unwrap().sprites[self.sprite_shown as usize].1.clone()
    }
}
