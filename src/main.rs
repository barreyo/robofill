
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate ggez;
extern crate specs;
#[macro_use] extern crate specs_derive;

mod board;
mod iso_coords;
mod components;
mod resources;

use ggez::{conf, GameResult, Context, graphics, timer};
use ggez::event::*;
use specs::{World, Dispatcher, DispatcherBuilder};

use std::time::Duration;

use board::Board;

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
        dispatcher_builder = components::input::init_world(&mut world, dispatcher_builder);

        let dispatcher = dispatcher_builder.build();

        // Player entity
        world.create_entity()
            .with(components::input::Controllable)
            .build();

        let s = MainState {
            world: world,
            dispatcher: dispatcher,
        };
        Ok(s)
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
        graphics::clear(ctx);
        graphics::present(ctx);
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
    //    fn resize_event(&mut self, ctx: &mut Context, width: u32, height: u32) {
    // graphics::set_screen_coordinates(ctx, 0.0, width as f32, 0.0, height as f32).unwrap();
    // }
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
