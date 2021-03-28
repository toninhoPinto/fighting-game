use std::collections::HashMap;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::JoystickSubsystem;
use sdl2::GameControllerSubsystem;
use sdl2::joystick::Joystick;
use sdl2::controller::GameController;

pub enum Controller {
    Keyboard,
    Controller(GameController),
    Joystick(Joystick) //sdl2::joystick::Joystick
}

pub fn handle_new_controller(controller: &GameControllerSubsystem, joystick: &JoystickSubsystem, event: &Event, joys: &mut HashMap<u32, Controller>) {
    match *event {
        Event::JoyDeviceAdded { which, .. } => {
            println!("added joystick: {}", which);
            let joy = joystick.open(which as u32).unwrap();
            joys.insert(which, Controller::Joystick(joy));
        },
        Event::ControllerDeviceAdded { which, ..} => {
            println!("added controller: {}", which);
            let control = controller.open(which as u32).unwrap();
            joys.insert(which, Controller::Controller(control));
        },
        Event::Quit { .. } |
        Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
            //break 'running
        },
        _ => {}
    }
}