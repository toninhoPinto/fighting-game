use sdl2::render::Texture;
use std::cmp;

pub struct Animation<'a>{
    pub name: String,
    pub speed: f64,
    pub length: i32,
    sprites: Vec<Texture<'a>>
}

impl<'a> Animation<'a> {
    pub fn new(sprites: Vec<Texture<'a>>, name: String, speed: f64) -> Self {
        Self {
            name,
            speed,
            length: sprites.len() as i32,
            sprites
        }
    }
}

pub struct Animator<'a>{
    pub animation_index: f64,
    pub current_animation: Option<&'a Animation<'a>>,
    pub is_playing: bool,
    play_once: bool,
    rewind: bool,
}

impl<'a> Animator<'a> {
    pub fn new() -> Self {
        Self {
            animation_index: 0.0,
            current_animation: None,
            is_playing: false,
            play_once: false,
            rewind: false,
        }
    }

    pub fn play(&mut self, new_animation: &'a Animation<'a>){
        if self.current_animation.is_none() ||
        (self.current_animation.unwrap().name != new_animation.name) {
            println!("play animation repeat {}", new_animation.name);
            self.animation_index = 0.0;
            self.current_animation = Some(new_animation);
            self.play_once = false;
            self.is_playing = true;
            self.rewind = false;
        }
    }

    pub fn play_once(&mut self, new_animation: &'a Animation<'a>, play_rewind: bool){
        if self.current_animation.is_none() ||
        (self.current_animation.unwrap().name != new_animation.name) {
            println!("play animation once {} with rewind {}", new_animation.name, play_rewind);
            if !play_rewind {
                self.animation_index = 0.0;
            } else {
                self.animation_index = (new_animation.length - 1) as f64;
            }
            self.current_animation = Some(new_animation);
            self.play_once = true;
            self.is_playing = true;
            self.rewind = play_rewind;
        }
    }

    pub fn render(&mut self, debug: bool) -> &Texture{
        let playing_animation = self.current_animation.unwrap();
        if debug {
            println!("render start index {} for {}", self.animation_index, playing_animation.name);
        }

        if !self.rewind {
            self.animation_index = self.animation_index + playing_animation.speed;
            if debug {
                println!("render not rewind index {} for {}", self.animation_index, playing_animation.name);
            }
            if self.play_once {
                if self.animation_index >= (playing_animation.length - 1) as f64 {
                    self.animation_index = (playing_animation.length - 1) as f64;
                    self.is_playing = false;
                }
            } else {
                self.animation_index = self.animation_index % playing_animation.length as f64;
            }
        } else {
            self.animation_index = self.animation_index - playing_animation.speed;
            if debug {
                println!("render with rewind index {} for {}", self.animation_index, playing_animation.name);
            }
            if self.play_once {
                if self.animation_index < 0.0 {
                    self.animation_index = 0.0;
                    self.is_playing = false;
                }
            } else {
                self.animation_index = self.animation_index.abs() % playing_animation.length as f64
            }
        }

        if debug {
            println!("render with index {} for {}", self.animation_index, playing_animation.name);
        }
        &playing_animation.sprites[self.animation_index as usize]
    }

}