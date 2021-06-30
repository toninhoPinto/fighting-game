use sdl2::rect::Rect;

pub mod button_trait;
pub mod nav_button;
pub mod store_button;

pub struct ButtonUI {
    pub rect: Rect,
    pub is_pressed: bool,
    pub text: Option<String>,
    pub sprite: String,
    pub pressed_sprite: Option<String>,
}