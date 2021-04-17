use crate::{
    input::{controller_handler::Controller, translated_inputs::TranslatedInput},
    GameStateData,
};
use sdl2::{
    render::{Canvas, TextureCreator},
    video::{Window, WindowContext},
    EventPump, GameControllerSubsystem, JoystickSubsystem,
};
use std::collections::HashMap;

pub(crate) trait Scene {
    fn run(
        &mut self,
        game_state_stack: &mut Vec<Box<dyn Scene>>,
        game_state_data: &mut GameStateData,
        texture_creator: &TextureCreator<WindowContext>,
        event_pump: &mut EventPump,
        joystick: &JoystickSubsystem,
        controller: &GameControllerSubsystem,
        controls: &HashMap<String, TranslatedInput>,
        joys: &mut Controller,
        canvas: &mut Canvas<Window>,
    );
}
