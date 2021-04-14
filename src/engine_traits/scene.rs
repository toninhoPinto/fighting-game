use std::collections::HashMap;
use sdl2::{EventPump, GameControllerSubsystem, JoystickSubsystem, render::{Canvas, TextureCreator}, video::{Window, WindowContext}};
use crate::{GameStateData, input::{controller_handler::Controller, translated_inputs::TranslatedInput}};

pub(crate) trait Scene {

    fn run(&mut self, 
        game_state_stack: &mut Vec<Box<dyn Scene>>,
        game_state_data: &mut GameStateData,
        texture_creator: &TextureCreator<WindowContext>,
        event_pump: &mut EventPump, joystick: &JoystickSubsystem,
        controller: &GameControllerSubsystem,
        controls: &HashMap<String, TranslatedInput>,
        joys: &mut Controller,
        canvas: &mut Canvas<Window>);
}
