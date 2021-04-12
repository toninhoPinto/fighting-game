//main menu
    //story mode
    //arcade
    //versus
        //local
        //online
    //training mode
    //settings
    //credits
    //quit

use std::collections::HashMap;
use sdl2::{EventPump, GameControllerSubsystem, JoystickSubsystem, render::{Canvas, TextureCreator}, video::{Window, WindowContext}};
use crate::input::{controller_handler::Controller, translated_inputs::TranslatedInput};

//character select
//stage select
use crate::engine_traits::scene::Scene;

pub enum MenuScreen {
    MainMenu,
    VersusMenu,
    CharacterSelect,
    StageSelect,
    Settings,
    Credits,
}

pub struct MenuScene {
    curr_screen: MenuScreen,
    prev_screen: MenuScreen,
}

impl Scene for MenuScene {
    fn run(&mut self, 
        texture_creator: &TextureCreator<WindowContext>,
        event_pump: &mut EventPump, joystick: &JoystickSubsystem,
        controller: &GameControllerSubsystem,
        controls: &HashMap<String, TranslatedInput>,
        joys: &mut HashMap<u32, Controller>,
        canvas: &mut Canvas<Window>){
        //receive inputs for managing selecting menu options
        //update
        //render
    }
}
