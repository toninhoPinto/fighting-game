use sdl2::render::Texture;

pub struct Animation<'a> {
    pub name: String,
    pub length: i32,
    pub sprites: Vec<(i32, Texture<'a>)>,
}

impl<'a> Animation<'a> {
    pub fn new(sprites: Vec<(i32, Texture<'a>)>, name: String) -> Self {
        Self {
            name,
            length: sprites[sprites.len() - 1].0,
            sprites,
        }
    }
}

pub enum AnimationState {
    Startup,
    Active,
    Recovery,
}

pub struct Animator<'a> {
    pub animation_index: f64,
    pub sprite_shown: usize,
    pub speed: f64,
    pub current_animation: Option<&'a Animation<'a>>,
    pub is_playing: bool,
    pub is_finished: bool,
    pub play_once: bool,
    pub rewind: bool,
}

impl<'a> Animator<'a> {
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

    pub fn play(&mut self, new_animation: &'a Animation<'a>, speed: f64, play_rewind: bool) {
        self.play_animation(new_animation, speed, play_rewind,  false);
    }

    pub fn play_once(&mut self, new_animation: &'a Animation<'a>, speed: f64, play_rewind: bool) {
        self.play_animation(new_animation, speed, play_rewind,  true);
    }

    fn play_animation(&mut self, new_animation: &'a Animation<'a>, speed: f64, play_rewind: bool, play_once: bool){
        if !play_rewind {
            self.animation_index = 0.0;
            self.sprite_shown = 0;
        } else {
            self.animation_index = (new_animation.length - 1) as f64;
            self.sprite_shown = new_animation.sprites.len() - 1;
        }
        self.current_animation = Some(new_animation);
        self.play_once = play_once;
        self.is_playing = true;
        self.is_finished = false;
        self.rewind = play_rewind;
        self.speed = speed;
    }

    fn finished_animation(&mut self) {
        self.is_playing = false;
        self.is_finished = true;
    }

    //needs to receive a dt and then compare with animation speed or something
    pub fn update(&mut self) {
        let playing_animation = self.current_animation.unwrap();

        if self.is_playing {
            if !self.rewind {
                self.animation_index += self.speed;
            } else {
                self.animation_index -= self.speed;
            }

            if !self.rewind {
                if (playing_animation.sprites[self.sprite_shown].0 as f64) <= self.animation_index {
                    self.sprite_shown += 1;
                }
            } else {
                if (playing_animation.sprites[self.sprite_shown].0 as f64) > self.animation_index {
                    self.sprite_shown -= 1;
                }
            }
            

            if self.play_once {
                if !self.rewind {
                    if self.animation_index >= playing_animation.length as f64 {
                        self.animation_index = playing_animation.length as f64;
                        self.sprite_shown = playing_animation.sprites.len() - 1;
                        self.finished_animation();
                    }
                } else {
                    if self.rewind && self.animation_index < 0.0 {
                        self.animation_index = 0.0;
                        self.sprite_shown = 0;
                        self.finished_animation();
                    }
                }
            } else {
                self.animation_index = self.animation_index.abs() % playing_animation.length as f64;
                self.sprite_shown %= playing_animation.sprites.len();
            }
        }
    }

    pub fn render(&mut self) -> &Texture {
        let playing_animation = self.current_animation.unwrap();
        &playing_animation.sprites[self.sprite_shown as usize].1
    }
}
