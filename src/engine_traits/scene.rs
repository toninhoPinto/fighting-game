use std::collections::HashMap;
use sdl2::{EventPump, GameControllerSubsystem, JoystickSubsystem, render::{Canvas, TextureCreator}, video::{Window, WindowContext}};
use crate::input::{controller_handler::Controller, translated_inputs::TranslatedInput};

pub(crate) trait Scene {
    fn run(&mut self, 
        texture_creator: &TextureCreator<WindowContext>,
        event_pump: &mut EventPump, joystick: &JoystickSubsystem,
        controller: &GameControllerSubsystem,
        controls: &HashMap<String, TranslatedInput>,
        joys: &mut HashMap<u32, Controller>,
        canvas: &mut Canvas<Window>);
}
