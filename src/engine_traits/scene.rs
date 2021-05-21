use crate::{GameStateData, Transition, input::{input_devices::InputDevices}};
use sdl2::{
    render::{Canvas, TextureCreator},
    video::{Window, WindowContext},
    EventPump,
};
pub trait Scene {
    fn run(
        &mut self,
        game_state_data: &mut GameStateData,
        texture_creator: &TextureCreator<WindowContext>,
        event_pump: &mut EventPump,
        input_devices: &mut InputDevices,
        canvas: &mut Canvas<Window>,
    ) -> Transition;
}
