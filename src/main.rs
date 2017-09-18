
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate cgmath;
extern crate ggez;
extern crate specs;

mod board;

use ggez::*;

use specs::World;

use std::time::Duration;

use board::Board;

struct MainState {
    world: World,
    board: Board,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let s = MainState {
            world: World::new(),
            board: Board::new(10, 10, 50.0),
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context, _dt: Duration) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        let win_rect = graphics::get_screen_coordinates(ctx);
        self.board.render(ctx);

        // let win_w = win_rect.w;
        // let win_h = -win_rect.h;
        // let cell_size = 0.9 * win_h / self.grid_size as f32;
        // let starting_x = win_w / 2.0 - cell_size * self.grid_size as f32 / 2.0;
        // let starting_y = 50.0;
        //
        // graphics::set_color(ctx, graphics::Color::new(0.73, 0.88, 0.06, 1.0))?;
        //
        // for row in 0..self.grid_size {
        // for sq in 0..self.grid_size {
        // graphics::rectangle(ctx,
        // DrawMode::Fill,
        // graphics::Rect::new(row as f32 * cell_size + starting_x,
        // sq as f32 * cell_size + starting_y,
        // cell_size - 3.0,
        // cell_size - 3.0))?;
        // }
        // }
        //
        graphics::present(ctx);
        Ok(())
    }

    //    fn resize_event(&mut self, ctx: &mut Context, width: u32, height: u32) {
    // graphics::set_screen_coordinates(ctx, 0.0, width as f32, 0.0, height as f32).unwrap();
    // }
}

pub fn main() {
    let mut c = conf::Conf::new();

    c.window_width = 1000;
    c.window_height = 800;
    c.vsync = true;

    let ctx = &mut Context::load_from_conf("super_simple", "ggez", c).unwrap();
    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}
