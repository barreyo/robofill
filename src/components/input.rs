
use std::collections::HashMap;
use ggez::event::Keycode;

use specs::{System, HashMapStorage, VecStorage, Fetch, ReadStorage, WriteStorage, World,
            DispatcherBuilder};

use components::graphics::GameBoard;
use core::grid;
use components::positioning::{Direction, Animating, Position, TargetPosition};

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
     Fetch<'a, GameBoard>,
     ReadStorage<'a, InputMapping>,
     ReadStorage<'a, Controllable>,
     ReadStorage<'a, Position>,
     WriteStorage<'a, Animating>,
     WriteStorage<'a, TargetPosition>,
     WriteStorage<'a, Direction>);

    fn run(&mut self,
           (keyboard_input, board, input_mapping, controllable, position, mut animating, mut target, mut direction): Self::SystemData) {
        use specs::Join;

        let keyboard_input = &*keyboard_input;
        let board = &board.0;

        // TODO: Direction should probably be exclusive in that sense that,
        //       we can only go up, only down, only left or only right.
        //
        //       Diagonal movement should be invalid.
        let mut dir = grid::GridDirection::DirectionNorth;
        let mut btn_update = false;

        for mapping in input_mapping.join() {

            for (action, &code) in &mapping.0 {
                match *action {
                    InputAction::MoveUp => {
                        if keyboard_input.is_pressed(code) {
                            dir = grid::GridDirection::DirectionNorth;
                            btn_update = true;
                        }
                    }
                    InputAction::MoveDown => {
                        if keyboard_input.is_pressed(code) {
                            dir = grid::GridDirection::DirectionSouth;
                            btn_update = true;
                        }
                    }
                    InputAction::MoveRight => {
                        if keyboard_input.is_pressed(code) {
                            dir = grid::GridDirection::DirectionEast;
                            btn_update = true;
                        }
                    }
                    InputAction::MoveLeft => {
                        if keyboard_input.is_pressed(code) {
                            dir = grid::GridDirection::DirectionWest;
                            btn_update = true;
                        }
                    }
                }
            }
        }

        for (_c, pos, is_anim, d, tar) in (&controllable, &position, &mut animating, &mut direction, &mut target).join() {
            if !is_anim.0 && btn_update {
                d.0 = dir;

                let cur_tile = board.get_tile_coordinates(pos.0);
                let next_tile = board.get_neighbour(cur_tile, dir);

                match next_tile {
                    Some(c) => {
                        tar.0 = Some(board.get_tile_center_world_coordinate(c));
                        is_anim.0 = true ;
                    },
                    None => is_anim.0 = false,
                }
            }
        }
    }
}

pub fn init_world<'a, 'b>(world: &mut World,
                          dispatcher_builder: DispatcherBuilder<'a, 'b>)
                          -> DispatcherBuilder<'a, 'b> {
    world.register::<Controllable>();

    // Create the resource so we can add input to the map.
    world.add_resource(KeyboardInput::new());
    world.register::<InputMapping>();

    dispatcher_builder.add(Control, "Control", &[])
}
