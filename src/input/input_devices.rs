use std::collections::HashMap;

use sdl2::{GameControllerSubsystem, JoystickSubsystem};

use super::{controller_handler::Controller, translated_inputs::TranslatedInput};

pub struct InputDevices {
    pub joystick: JoystickSubsystem,
    pub  controller: GameControllerSubsystem,
    pub  controls: HashMap<String, TranslatedInput>,
    pub  joys: Controller,
}