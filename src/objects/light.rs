use amethyst::renderer::light::{Light, PointLight};
use amethyst::renderer::palette::rgb::Srgb;
use amethyst::ecs::{World, WorldExt};
use amethyst::core::Transform;

use amethyst::prelude::*;

pub fn init_light(world: &mut World) {
    let light: Light = PointLight {
        intensity: 30.0,
        radius: 1.0,
        color: Srgb::new(1.0, 1.0, 1.0),
        ..Default::default()
    }
    .into();

    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, 5.0);

    world.create_entity()
        .with(light)
        .with(transform)
        .build();
}
