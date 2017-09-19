
use std::collections::HashMap;
use ggez::event::Keycode;

use specs::{System, HashMapStorage, Fetch, ReadStorage, WriteStorage, World, DispatcherBuilder};

/// Holds all keypresses
pub struct KeyboardInput(pub HashMap<Keycode, bool>);

impl KeyboardInput {
    /// Create a new input structure
    pub fn new() -> Self {
        KeyboardInput(HashMap::new())
    }

    /// Check if a specific button is currently pressed
    pub fn is_pressed(&self, btn: Keycode) -> bool {
        match self.0.get(&btn) {
            Some(b) => *b,
            None => false,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum InputAction {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
}

pub struct InputMapping(pub HashMap<InputAction, Keycode>);

impl InputMapping {
    pub fn default() -> Self {
        let mut map = HashMap::new();

        map.insert(InputAction::MoveUp, Keycode::W);
        map.insert(InputAction::MoveDown, Keycode::S);
        map.insert(InputAction::MoveLeft, Keycode::A);
        map.insert(InputAction::MoveRight, Keycode::D);

        InputMapping(map)
    }
}

#[derive(Component, Debug)]
#[component(HashMapStorage)]
pub struct Controllable;

pub struct Control;

impl<'a> System<'a> for Control {
    type SystemData = (Fetch<'a, KeyboardInput>,
     Fetch<'a, InputMapping>,
     ReadStorage<'a, Controllable>);
    //  WriteStorage<'a, Velocity>);

    fn run(&mut self, (keyboard_input, input_mapping, _controllable): Self::SystemData) {
        let keyboard_input = &*keyboard_input;

        let mut _xvel = 0.0;
        let mut _yvel = 0.0;

        for (action, &code) in input_mapping.0.iter() {

            match action {
                &InputAction::MoveUp => {
                    if keyboard_input.is_pressed(code) {
                        println!("Moving UP!");
                    }
                }
                &InputAction::MoveDown => {
                    if keyboard_input.is_pressed(code) {
                        println!("Moving DOWN!");
                    }
                }
                &InputAction::MoveRight => {
                    if keyboard_input.is_pressed(code) {
                        println!("Moving RIGHT!");
                    }
                }
                &InputAction::MoveLeft => {
                    if keyboard_input.is_pressed(code) {
                        println!("Moving LEFT!");
                    }
                }
            }
        }
    }
}

pub fn init_world<'a, 'b>(world: &mut World,
                          dispatcher_builder: DispatcherBuilder<'a, 'b>)
                          -> DispatcherBuilder<'a, 'b> {
    world.register::<Controllable>();
    world.add_resource(KeyboardInput::new());
    world.add_resource(InputMapping::default());

    dispatcher_builder.add(Control, "Control", &[])
}
