use std::collections::HashMap;

use sdl2::controller::GameController;
use sdl2::event::Event;
use sdl2::joystick::Joystick;
use sdl2::keyboard::Keycode;
use sdl2::GameControllerSubsystem;
use sdl2::JoystickSubsystem;

const KEYBOARD_ID: i32 = 666;

pub enum ControllerType {
    Keyboard,
    Controller(GameController),
    Joystick(Joystick),
}

impl PartialEq for ControllerType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ControllerType::Keyboard, ControllerType::Keyboard) => true,

            (ControllerType::Controller(c1), ControllerType::Controller(c2)) => {
                c1.instance_id() == c2.instance_id()
            }

            (ControllerType::Joystick(j1), ControllerType::Joystick(j2)) => {
                j1.instance_id() == j2.instance_id()
            }

            (_, _) => false,
        }
    }
}

pub struct Controller {
    pub connected_controllers: HashMap<u32, ControllerType>,
    pub selected_controllers: [Option<u32>; 2],
    pub controller_id: i32,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            connected_controllers: HashMap::new(),
            selected_controllers: [None, None],
            controller_id: -1,
        }
    }

    pub fn add_keyboard(&mut self) {
        self.connected_controllers
            .insert(KEYBOARD_ID as u32, ControllerType::Keyboard);
        self.selected_controllers[0] = Some(KEYBOARD_ID as u32);
    }

    pub fn add(&mut self, controller: ControllerType) {
        self.controller_id += 1;
        let id = self.controller_id as u32;
        self.connected_controllers.insert(id, controller);

        match self.selected_controllers[0] {
            Some(p1_id) => {
                if p1_id == KEYBOARD_ID as u32 {
                    self.selected_controllers[1] = self.selected_controllers[0];
                    self.selected_controllers[0] = Some(id);
                } else {
                    self.selected_controllers[1] = Some(id);
                }
            }
            None => {
                self.selected_controllers[0] = Some(id);
            }
        }
    }

    pub fn remove(&mut self, id_controller: u32) {
        if self.connected_controllers.contains_key(&id_controller) {
            self.connected_controllers.remove(&id_controller);

            match self.selected_controllers[0] {
                Some(c) if c == id_controller => {
                    self.selected_controllers[0] = None;
                }
                _ => {}
            }

            match self.selected_controllers[1] {
                Some(c) if c == id_controller => {
                    self.selected_controllers[1] = None;
                }
                _ => {}
            }
        }
    }
}

pub fn handle_new_controller(
    controller: &GameControllerSubsystem,
    joystick: &JoystickSubsystem,
    event: &Event,
    controllers: &mut Controller,
) {
    match *event {
        Event::JoyDeviceAdded { which, .. } => {
            println!("added joystick: {}", which);
            let joy = joystick.open(which as u32).unwrap();
            controllers.add(ControllerType::Joystick(joy));
        }
        Event::ControllerDeviceAdded { which, .. } => {
            println!("added controller: {}", which);
            let control = controller.open(which as u32).unwrap();
            controllers.add(ControllerType::Controller(control));
        }
        Event::JoyDeviceRemoved { which, .. } => {
            println!("removed joystick: {}", which);
            controllers.remove(which);
        }
        Event::ControllerDeviceRemoved { which, .. } => {
            println!("removed controller: {}", which);
            controllers.remove(which);
        }
        Event::Quit { .. }
        | Event::KeyDown {
            keycode: Some(Keycode::Escape),
            ..
        } => {
            //break 'running
        }
        _ => {}
    }
}
