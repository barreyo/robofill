
use specs::{System, VecStorage, HashMapStorage, Fetch, ReadStorage, WriteStorage, World,
            DispatcherBuilder};
use cgmath::Vector2;

use resources::delta_time;
use components::graphics::GameBoard;
use core::grid::{GridDirection, GridCoordinate};

#[derive(Component, Debug)]
#[component(VecStorage)]
pub struct Position(pub Vector2<f32>);

#[derive(Component, Debug)]
#[component(VecStorage)]
pub struct TargetPosition(pub Option<Vector2<f32>>);

#[derive(Component, Debug)]
#[component(VecStorage)]
pub struct GridPosition(pub GridCoordinate);

#[derive(Component, Debug)]
#[component(VecStorage)]
pub struct Velocity(pub Vector2<f32>);

#[derive(Component, Debug)]
#[component(VecStorage)]
pub struct Direction(pub GridDirection);

#[derive(Component, Debug)]
#[component(VecStorage)]
pub struct Animating(pub bool);

#[derive(Component, Debug)]
#[component(VecStorage)]
pub struct AnimationTime(pub f32);

pub struct Move;

impl<'a> System<'a> for Move {
    type SystemData = (Fetch<'a, delta_time::DeltaTime>,
     Fetch<'a, GameBoard>,
     ReadStorage<'a, Velocity>,
     WriteStorage<'a, Animating>,
     WriteStorage<'a, AnimationTime>,
     WriteStorage<'a, GridPosition>,
     WriteStorage<'a, TargetPosition>,
     WriteStorage<'a, Position>);

    fn run(&mut self, (delta, board, vel, mut ani, mut ani_time, mut grid_pos, mut tar_pos, mut pos): Self::SystemData) {
        use specs::Join;

        let delta = delta.0;
        let board = &board.0;

        for (vel, is_anim, ani_time, gp, tp, pos) in (&vel, &mut ani, &mut ani_time, &mut grid_pos, &mut tar_pos, &mut pos).join() {
            if is_anim.0 {
                ani_time.0 += vel.0.x * delta;

                let starting_coords = board.get_tile_center_world_coordinate(gp.0);
                pos.0.x = (1.0 - ani_time.0) * starting_coords.x + ani_time.0 * tp.0.unwrap().x;
                pos.0.y = (1.0 - ani_time.0) * starting_coords.y + ani_time.0 * tp.0.unwrap().y;

                if ani_time.0 > 1.0 {
                    is_anim.0 = false;
                    ani_time.0 = 0.0;
                    gp.0 = board.get_tile_coordinates(pos.0);
                    tp.0 = None;

                    println!("Grid pos: {:?}", gp.0);
                    println!("Pos: {:?}", pos.0);
                }
            }
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
    world.register::<GridPosition>();
    world.register::<TargetPosition>();
    world.register::<Velocity>();
    world.register::<Animating>();
    world.register::<AnimationTime>();
    world.register::<Direction>();

    // Movement happen before we snap into grid cells
    dispatcher_builder.add(Move, "Move", &[])
}
