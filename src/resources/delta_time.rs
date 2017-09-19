
use specs::{World, DispatcherBuilder};

pub struct DeltaTime(pub f32);

pub fn init_world<'a, 'b>(world: &mut World,
                          dispatcher_builder: DispatcherBuilder<'a, 'b>)
                          -> DispatcherBuilder<'a, 'b> {
    world.add_resource(DeltaTime(0.05));
    dispatcher_builder
}
