use sdl2::rect::Rect;
use splines::{Interpolation, Key, Spline};

pub struct SimpleAnimator {
    pub animation_index: f64,
    pub speed: f64,
    pub transformations: Vec<Box<dyn AnimationTransformation>>,
    pub is_starting: bool,
    pub is_playing: bool,
    pub is_finished: bool,
    pub play_once: bool,
}

impl SimpleAnimator {
    pub fn new(transformations: Vec<Box<dyn AnimationTransformation>>) -> Self {
        Self {
            animation_index: 0.0,
            speed: 1.0,
            transformations,
            is_starting: true,
            is_playing: false,
            is_finished: false,
            play_once: false,
        }
    }

    pub fn play(&mut self, speed: f64) {
        self.play_animation( speed,  false);
    }

    pub fn play_once(&mut self, speed: f64) {
        self.play_animation(speed,  true);
    }

    pub fn reset(&mut self) {
        self.animation_index = 0f64;
    }

    pub fn reset_full(&mut self, rect: &mut Rect) {
        self.animation_index = 0f64;
        for transformation in self.transformations.iter() {
            transformation.transform(rect, self.animation_index);
        }
    }

    pub fn play_animation(&mut self, speed: f64, play_once: bool) {
            self.play_once = play_once;
            self.is_starting = true;
            self.is_playing = true;
            self.is_finished = false;
            self.speed = speed;
    }

    fn finished_animation(&mut self) {
        self.is_playing = false;
        self.is_finished = true;
    }

    pub fn update(&mut self, rect: &mut Rect, time: f64) {
        if self.is_playing {
            self.animation_index += time * self.speed;
            
            let mut is_playing = true;
            for transformation in self.transformations.iter() {
                is_playing &= transformation.transform(rect, self.animation_index);
            }
            self.is_playing = is_playing;
        }
    }

}

pub trait AnimationTransformation {
    fn transform(&self, rect: &mut Rect, time: f64) -> bool;
}

pub struct ScaleAnim {
    pub original_size: (u32, u32),
    pub spline: Spline<f64, f64>,
}

impl AnimationTransformation for ScaleAnim {
    fn transform(&self, rect: &mut Rect, time: f64) -> bool {
        rect.set_width((self.original_size.0  as f64 * self.spline.clamped_sample(time).unwrap()) as u32);
        rect.set_height((self.original_size.1 as f64 * self.spline.clamped_sample(time).unwrap()) as u32);
        return time <= self.spline.keys()[1].t;
    }
}

pub struct MoveAnim {
    pub original_pos: (i32, i32),
    pub offset_x: i32,
    pub offset_y: i32,
    pub spline: Spline<f64, f64>,
}

impl AnimationTransformation for MoveAnim {
    fn transform(&self, rect: &mut Rect, time: f64) -> bool {
        rect.set_x((self.original_pos.0 as f64 + self.offset_x as f64 * self.spline.clamped_sample(time).unwrap()) as i32);
        rect.set_y((self.original_pos.1 as f64 + self.offset_y as f64 * self.spline.clamped_sample(time).unwrap()) as i32);
        return time <= self.spline.keys()[1].t;
    }
}

pub fn init_combo_animation(original_rect: Rect) -> SimpleAnimator{
    let mut transformations: Vec<Box<dyn AnimationTransformation>> = Vec::new();

    let start = Key::new(0., 1., Interpolation::Bezier(3.0f64));
    let end = Key::new(1., 2., Interpolation::default()); //second interpolation is not used
    let spline = Spline::from_vec(vec![start, end]);

    transformations.push(Box::new(ScaleAnim {
        original_size: (original_rect.width(), original_rect.height()),
        spline: spline.clone(),
    }));

    let start = Key::new(0., 0., Interpolation::Bezier(3.0f64));
    let end = Key::new(1., 1., Interpolation::default()); //second interpolation is not used
    let spline = Spline::from_vec(vec![start, end]);

    transformations.push(Box::new(MoveAnim {
        original_pos: (original_rect.x(), original_rect.y()),
        offset_x: original_rect.width() as i32 / 4,
        offset_y: -(original_rect.height() as i32) / 2,
        spline: spline.clone(),
    }));

    SimpleAnimator::new(transformations)
}
