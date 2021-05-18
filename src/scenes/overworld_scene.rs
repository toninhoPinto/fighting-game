pub struct OverworldScene<'a> {
    pub curr_screen: MenuScreen,
    pub prev_screen: Option<MenuScreen>,
}


impl<'a> Scene for MenuScene<'a> {
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
    ) {



    }
}