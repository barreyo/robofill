
use std::collections::HashMap;
use ggez::event::Keycode;

use specs::{System, HashMapStorage, VecStorage, Fetch, ReadStorage, WriteStorage, World,
            DispatcherBuilder};

use components::positioning::Velocity;

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

#[derive(Component, Debug)]
#[component(VecStorage)]
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
     ReadStorage<'a, InputMapping>,
     ReadStorage<'a, Controllable>,
     WriteStorage<'a, Velocity>);

    fn run(&mut self,
           (keyboard_input, input_mapping, controllable, mut velocity): Self::SystemData) {
        use specs::Join;

        let keyboard_input = &*keyboard_input;

        let mut xvel = 0.0;
        let mut yvel = 0.0;

        for mapping in input_mapping.join() {

            for (action, &code) in &mapping.0 {
                match *action {
                    InputAction::MoveUp => {
                        if keyboard_input.is_pressed(code) {
                            yvel -= 100.0;
                        }
                    }
                    InputAction::MoveDown => {
                        if keyboard_input.is_pressed(code) {
                            yvel += 100.0;
                        }
                    }
                    InputAction::MoveRight => {
                        if keyboard_input.is_pressed(code) {
                            xvel += 100.0;
                        }
                    }
                    InputAction::MoveLeft => {
                        if keyboard_input.is_pressed(code) {
                            xvel -= 100.0;
                        }
                    }
                }
            }
        }

        for (_c, vel) in (&controllable, &mut velocity).join() {
            vel.0.x = xvel;
            vel.0.y = yvel;
        }
    }
}

pub fn init_world<'a, 'b>(world: &mut World,
                          dispatcher_builder: DispatcherBuilder<'a, 'b>)
                          -> DispatcherBuilder<'a, 'b> {
    world.register::<Controllable>();
    world.add_resource(KeyboardInput::new());
    world.register::<InputMapping>();

    dispatcher_builder.add(Control, "Control", &[])
}
