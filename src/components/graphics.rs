
use specs::{VecStorage, World, DispatcherBuilder};
use ggez::graphics::Image;

use core::grid::Board;

#[derive(Component, Debug)]
#[component(VecStorage)]
pub struct RenderableSprite(pub Image);

pub struct GameBoard(pub Board);

pub fn init_world<'a, 'b>(world: &mut World,
                          dispatcher_builder: DispatcherBuilder<'a, 'b>)
                          -> DispatcherBuilder<'a, 'b> {
    world.register::<RenderableSprite>();

    dispatcher_builder
}
