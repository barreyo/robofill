
use specs::{System, VecStorage, Fetch, ReadStorage, WriteStorage, World, DispatcherBuilder};
use cgmath::Vector2;

use resources::delta_time;

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


pub fn init_world<'a, 'b>(world: &mut World,
                          dispatcher_builder: DispatcherBuilder<'a, 'b>)
                          -> DispatcherBuilder<'a, 'b> {
    world.register::<Position>();
    world.register::<Velocity>();

    dispatcher_builder.add(Move, "Move", &[])
}
