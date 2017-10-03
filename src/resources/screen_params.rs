
use ggez::graphics::Rect;
use specs::{World, DispatcherBuilder};

pub struct ScreenParams(pub Rect);

pub fn init_world<'a, 'b>(world: &mut World,
                          dispatcher_builder: DispatcherBuilder<'a, 'b>)
                          -> DispatcherBuilder<'a, 'b> {
    world.add_resource(ScreenParams(Rect::new(0.0, 0.0, 0.0, 0.0)));
    dispatcher_builder
}
