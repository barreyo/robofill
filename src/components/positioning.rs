
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

#[derive(Component, Debug)]
#[component(VecStorage)]
pub struct Direction(pub Vector2<f32>);

#[derive(Component, Debug)]
#[component(VecStorage)]
pub struct Animating(pub bool);

pub struct Move;

impl<'a> System<'a> for Move {
    type SystemData = (Fetch<'a, delta_time::DeltaTime>,
     Fetch<'a, GameBoard>,
     ReadStorage<'a, Velocity>,
     ReadStorage<'a, Direction>,
     ReadStorage<'a, Animating>,
     WriteStorage<'a, Position>);

    fn run(&mut self, (delta, board, vel, dir, mut ani, mut pos): Self::SystemData) {
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

pub fn init_world<'a, 'b>(world: &mut World,
                          dispatcher_builder: DispatcherBuilder<'a, 'b>)
                          -> DispatcherBuilder<'a, 'b> {
    world.register::<Position>();
    world.register::<Velocity>();
    world.register::<Animating>();
    world.register::<Direction>();

    // Movement happen before we snap into grid cells
    dispatcher_builder.add(Move, "Move", &[])
}
