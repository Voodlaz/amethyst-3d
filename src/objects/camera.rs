use amethyst::prelude::*;
use amethyst::renderer::Camera;


pub fn init_camera(world: &mut World) {
    world.create_entity()
        .with(Camera::standard_3d(1024.0, 768.0))
        .build();
}
