
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate cgmath;
extern crate ggez;
extern crate specs;
#[macro_use] extern crate specs_derive;

mod core;
mod components;
mod resources;

use cgmath::Vector2;
use ggez::{conf, GameResult, Context, timer};
use ggez::graphics::*;
use ggez::event::*;
use specs::{World, Dispatcher, DispatcherBuilder};

use std::time::Duration;

use core::iso_coords::IsoCoord;

struct MainState<'a, 'b> {
    world: World,
    dispatcher: Dispatcher<'a, 'b>,
}

impl<'a, 'b> MainState<'a, 'b> {
    fn new(ctx: &mut Context) -> GameResult<MainState<'a, 'b>> {

        let screen_rect = ggez::graphics::get_screen_coordinates(ctx);

        // ECS world type
        let mut world = World::new();
        let mut dispatcher_builder = DispatcherBuilder::new();

        dispatcher_builder = resources::delta_time::init_world(&mut world, dispatcher_builder);
        dispatcher_builder = resources::screen_params::init_world(&mut world, dispatcher_builder);
        dispatcher_builder = components::graphics::init_world(&mut world, dispatcher_builder);
        dispatcher_builder = components::positioning::init_world(&mut world, dispatcher_builder);
        dispatcher_builder = components::input::init_world(&mut world, dispatcher_builder);

        let dispatcher = dispatcher_builder.build();

        let game_board = core::grid::Board::new(12, 12, 30.0, Vector2::new(screen_rect.w / 2.0, 0.0));
        let starting_pos = game_board.get_tile_center_world_coordinate([0, 0]);

        world.add_resource(components::graphics::GameBoard(game_board));

        // Player entity
        world.create_entity()
            .with(components::positioning::Position(starting_pos))
            .with(components::positioning::GridPosition([0, 0]))
            .with(components::positioning::Velocity(Vector2::new(5.0, 5.0)))
            .with(components::positioning::TargetPosition(None))
            .with(components::positioning::Animating(false))
            .with(components::positioning::AnimationTime(0.0))
            .with(components::positioning::Direction(core::grid::GridDirection::DirectionSouth))
            .with(components::input::Controllable)
            .with(components::input::InputMapping::default())
            .with(components::graphics::RenderableSprite(Image::new(ctx, "/sprites/duck.png")?))
            .build();

        Ok(MainState {
            world: world,
            dispatcher: dispatcher,
        })
    }
}

impl<'a, 'b> EventHandler for MainState<'a, 'b> {
    fn update(&mut self, _ctx: &mut Context, dt: Duration) -> GameResult<()> {

        {
            let mut delta = self.world.write_resource::<resources::delta_time::DeltaTime>();
            *delta = resources::delta_time::DeltaTime(dt.as_secs() as f32 + dt.subsec_nanos() as f32 * 1e-9);
        }

        self.dispatcher.dispatch(&mut self.world.res);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        use specs::Join;
        ggez::graphics::clear(ctx);

        let entities = self.world.entities();
        let positions = self.world.read::<components::positioning::Position>();
        let sprites = self.world.read::<components::graphics::RenderableSprite>();
        let grid = self.world.write_resource::<components::graphics::GameBoard>();

        grid.0.render(ctx)?;

        ggez::graphics::set_color(ctx, ggez::graphics::Color::new(1.0, 1.0, 1.0, 1.0))?;
        for (_entity, position, sprite) in (&*entities, &positions, &sprites).join() {
            let iso_coords = IsoCoord::from_cartesian(position.0.x, position.0.y);
            draw(ctx, &sprite.0, iso_coords.as_point(), 0.0)?;
        }

        ggez::graphics::present(ctx);

        // Keep for prototyping, do actual signaling to the OS for real product
        timer::sleep_until_next_frame(ctx, 60);
        Ok(())
    }

    fn key_down_event(&mut self, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        let mut keyboard_input = self.world.write_resource::<components::input::KeyboardInput>();
        keyboard_input.0.insert(keycode, true);
    }

    fn key_up_event(&mut self, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        let mut keyboard_input = self.world.write_resource::<components::input::KeyboardInput>();
        keyboard_input.0.insert(keycode, false);
    }

    fn resize_event(&mut self, ctx: &mut Context, width: u32, height: u32) {
        ggez::graphics::set_screen_coordinates(ctx, 0.0, width as f32, 0.0, height as f32).unwrap();

        {
            let mut params = self.world.write_resource::<resources::screen_params::ScreenParams>();
            *params = resources::screen_params::ScreenParams(Rect::new(0.0, width as f32, 0.0, height as f32));
        }
    }
}

pub fn main() {
    let mut c = conf::Conf::new();

    c.window_title = "robofill".to_string();
    c.window_width = 800;
    c.window_height = 800;
    c.resizable = true;
    c.vsync = true;

    let ctx = &mut Context::load_from_conf("robofill", "patar", c).unwrap();
    let state = &mut MainState::new(ctx).unwrap();

    run(ctx, state).unwrap();
}
