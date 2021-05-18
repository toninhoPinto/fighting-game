use crate::{GameStateData, input::{input_devices::InputDevices}};
use sdl2::{
    render::{Canvas, TextureCreator},
    video::{Window, WindowContext},
    EventPump,
};
pub(crate) trait Scene {
    fn run(
        &mut self,
        game_state_stack: &mut Vec<Box<dyn Scene>>,
        game_state_data: &mut GameStateData,
        texture_creator: &TextureCreator<WindowContext>,
        event_pump: &mut EventPump,
        input_devices: &mut InputDevices,
        canvas: &mut Canvas<Window>,
    );
}
