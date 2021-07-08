
use rand::Rng;
use sdl2::rect::Rect;

use crate::game_logic::characters::player::Player;


const AMPLITUDE: i8 = 6;

#[derive(Debug)]
pub struct Camera {
    pub rect: Rect,
    
    pub is_shaking: bool,

    pub shake_duration: i32,
    pub shake_frequency: i32,
    pub shake_time: i32, 
    pub shake_horizontal_samples: Vec<f64>,
    pub shake_vertical_samples: Vec<f64>,

    shaken_x: i32,
    shaken_y: i32,
}

impl Camera {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {

        let duration = 100; //milliseconds
        let frequency = 60; //Hertz

        Self {
            rect: Rect::new(x, y, width, height),
            is_shaking: false,
            
            shake_duration: duration,
            shake_frequency: frequency,
            shake_time: 0,
            shake_horizontal_samples: Vec::new(),
            shake_vertical_samples: Vec::new(),

            shaken_x: 0,
            shaken_y: 0, 
        }
    }

    pub fn get_camera(&self) -> Rect{
        return Rect::new(self.rect.x() + self.shaken_x, self.rect.y() + self.shaken_y, self.rect.width(), self.rect.height());
    }

    pub fn shake(&mut self) {

        let mut rng = rand::thread_rng();

        let sample_count = (self.shake_duration as f64/1000f64) * self.shake_frequency as f64;
        let mut samples = Vec::new();
        for _ in 0..sample_count as i32 {
            samples.push(rng.gen::<f64>() * 2f64 - 1f64);
        }
        self.shake_horizontal_samples = samples;

        let mut samples = Vec::new();
        for _ in 0..sample_count as i32 {
            samples.push(rng.gen::<f64>() * 2f64 - 1f64);
        }
        self.shake_vertical_samples = samples;

        self.shake_time = 0;
        self.is_shaking = true
    }

    pub fn update(&mut self, level_size: i32, player: &Player, dt: f64) {
        let mut proposed_x = player.position.x as i32 - self.rect.width() as i32 / 2;
        
        if proposed_x < 0 {
            proposed_x = 0;
        }

        if proposed_x + self.rect.width() as i32 > level_size {
            proposed_x = level_size - self.rect.width() as i32;
        }

        self.rect.set_x(proposed_x);

        if self.is_shaking {
            self.shake_time += (dt * 1000f64) as i32;
            self.is_shaking = self.shake_time <= self.shake_duration;

            if self.is_shaking {
                self.shaken_x = (self.amplitude(&self.shake_horizontal_samples) * AMPLITUDE as f64) as i32;
                self.shaken_y = (self.amplitude(&self.shake_vertical_samples) * AMPLITUDE as f64) as i32;
            } else {
                self.shaken_x = 0;
                self.shaken_y = 0;
            }
        }
    }

    pub fn amplitude(&self, samples: &Vec<f64>) -> f64 {
        if self.is_shaking {

            // Get the previous and next sample
            let s = self.shake_time as f64 / 1000f64 * self.shake_frequency as f64;
            let s0 = s.floor() as i32;
            let s1 = s0 + 1;
            
            // Get the current decay
            let k = self.decay(self.shake_time);
            
            // Return the current amplitude
            return (self.noise(s0, samples) + (s as i32 - s0) as f64 * (self.noise(s1, samples) - self.noise(s0, samples))) * k;
        } else {
            return 0f64;
        }
    }

    pub fn noise(&self, sample: i32, samples: &Vec<f64>) -> f64{
		// Retrieve the randomized value from the samples
		if sample >= samples.len() as i32 {return 0f64};
		return samples[sample as usize] as f64;
	}

    pub fn decay(&self, time: i32) -> f64{
        // Linear decay
        if time >= self.shake_duration { return 0f64 };
        return (self.shake_duration as f64 - time as f64) / self.shake_duration as f64;
    }


}
