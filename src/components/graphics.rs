
use specs::{VecStorage, World, DispatcherBuilder};
use ggez::graphics::Image;

use board::Board;

#[derive(Component, Debug)]
#[component(VecStorage)]
pub struct RenderableSprite(pub Image);

#[derive(Component, Debug)]
#[component(VecStorage)]
pub struct RenderableSpriteGrid(pub Board);

pub fn init_world<'a, 'b>(world: &mut World,
                          dispatcher_builder: DispatcherBuilder<'a, 'b>)
                          -> DispatcherBuilder<'a, 'b> {
    world.register::<RenderableSprite>();
    world.register::<RenderableSpriteGrid>();

    dispatcher_builder
}
