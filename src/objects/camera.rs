use amethyst::prelude::*;
use amethyst::renderer::Camera;
use amethyst::core::transform::*;

pub fn init_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, 10.0);

    world.create_entity()
        .with(Camera::standard_3d(500.0, 500.0))
        .with(transform)
        .build();
}
