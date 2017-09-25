
use specs::{System, VecStorage, HashMapStorage, Fetch, ReadStorage, WriteStorage, World,
            DispatcherBuilder};
use cgmath::Vector2;

use resources::delta_time;
use components::graphics::GameBoard;

#[derive(Component, Debug)]
#[component(VecStorage)]
pub struct Position(pub Vector2<f32>);

#[derive(Component, Debug)]
#[component(VecStorage)]
pub struct Velocity(pub Vector2<f32>);

pub struct Move;

impl<'a> System<'a> for Move {
    type SystemData = (Fetch<'a, delta_time::DeltaTime>,
     ReadStorage<'a, Velocity>,
     WriteStorage<'a, Position>);

    fn run(&mut self, (delta, vel, mut pos): Self::SystemData) {
        use specs::Join;

        let delta = delta.0;

        for (vel, pos) in (&vel, &mut pos).join() {
            pos.0 += vel.0 * delta;
        }
    }
}

#[derive(Component, Debug)]
#[component(HashMapStorage)]
pub struct Controllable;

#[derive(Component, Debug)]
#[component(HashMapStorage)]
pub struct GridSnapping;

pub struct Snapping;

impl<'a> System<'a> for Snapping {
    type SystemData = (WriteStorage<'a, Position>,
     ReadStorage<'a, GridSnapping>,
     Fetch<'a, GameBoard>);

    fn run(&mut self, (mut pos, _snapping, grid): Self::SystemData) {
        use specs::Join;

        let grid = &grid.0;

        for (pos, _s) in (&mut pos, &_snapping).join() {
            let grid_pos = grid.get_cell_coords(pos.0.x, pos.0.y);
            pos.0.x = grid_pos.x;
            pos.0.y = grid_pos.y;
            println!("Snapping to X: {}, Y: {}", pos.0.x, pos.0.y);
        }
    }
}

pub fn init_world<'a, 'b>(world: &mut World,
                          dispatcher_builder: DispatcherBuilder<'a, 'b>)
                          -> DispatcherBuilder<'a, 'b> {
    world.register::<Position>();
    world.register::<Velocity>();
    world.register::<GridSnapping>();

    // Movement happen before we snap into grid cells
    dispatcher_builder.add(Move, "Move", &[])
        .add(Snapping, "Snapping", &["Move"])
}
