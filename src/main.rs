
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate cgmath;
extern crate ggez;
extern crate specs;
#[macro_use] extern crate specs_derive;

mod board;
mod meta;
mod components;
mod resources;

use cgmath::Vector2;
use ggez::{conf, GameResult, Context, timer};
use ggez::graphics::*;
use ggez::event::*;
use specs::{World, Dispatcher, DispatcherBuilder};

use std::time::Duration;

struct MainState<'a, 'b> {
    world: World,
    dispatcher: Dispatcher<'a, 'b>,
}

impl<'a, 'b> MainState<'a, 'b> {
    fn new(ctx: &mut Context) -> GameResult<MainState<'a, 'b>> {

        // ECS world type
        let mut world = World::new();
        let mut dispatcher_builder = DispatcherBuilder::new();

        dispatcher_builder = resources::delta_time::init_world(&mut world, dispatcher_builder);
        dispatcher_builder = resources::screen_params::init_world(&mut world, dispatcher_builder);
        dispatcher_builder = components::graphics::init_world(&mut world, dispatcher_builder);
        dispatcher_builder = components::positioning::init_world(&mut world, dispatcher_builder);
        dispatcher_builder = components::input::init_world(&mut world, dispatcher_builder);

        let dispatcher = dispatcher_builder.build();

        // Baord entity
        world.create_entity()
            .with(components::positioning::Position(Vector2::new(0.0, 0.0)))
            .with(components::graphics::RenderableSpriteGrid(board::Board::new(12, 12, 30.0)))
            .build();

        // Player entity
        world.create_entity()
            .with(components::positioning::Position(Vector2::new(0.0, 0.0)))
            .with(components::positioning::Velocity(Vector2::new(0.0, 0.0)))
            .with(components::input::Controllable)
            .with(components::input::InputMapping::default())
            .with(components::graphics::RenderableSprite(Image::new(ctx, "/duck.png")?))
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
        let grids = self.world.read::<components::graphics::RenderableSpriteGrid>();

        for (_entity, _position, grid) in (&*entities, &positions, &grids).join() {
            grid.0.render(ctx);
        }

        for (_entity, position, sprite) in (&*entities, &positions, &sprites).join() {
            draw(ctx, &sprite.0, Point::new(position.0.x, position.0.y), 0.0)?;
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
    c.vsync = true;

    let ctx = &mut Context::load_from_conf("robofill", "patar", c).unwrap();
    let state = &mut MainState::new(ctx).unwrap();

    run(ctx, state).unwrap();
}
