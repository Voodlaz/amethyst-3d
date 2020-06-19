use amethyst::{
    assets::{Format as AssetFormat, Handle, Loader},
    core::{math::Vector3, Transform, TransformBundle},
    ecs::{World, WorldExt},
    error::Error,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        camera::Camera,
        light::{Light, PointLight},
        mtl::{Material, MaterialDefaults},
        palette::{Srgb, Srgba},
        plugins::{RenderShaded3D, RenderSkybox, RenderToWindow},
        rendy::{
            mesh::{MeshBuilder, Normal, Position, TexCoord},
            texture::palette::load_from_srgba,
        },
        types::{DefaultBackend, Mesh, MeshData},
        RenderingBundle,
    },
    utils::application_root_dir,
};

use std::fs::File;
use std::io::BufReader;
use obj::{load_obj, Obj};

mod objects;
use crate::objects::{camera, sphere};

struct MyState;

#[derive(Clone, Debug)]
struct Custom;

impl AssetFormat for Custom {
    fn name(&self) -> &'static str {
        "CUSTOM"
    }

    fn import_simple() -> Result<MeshData, Error> {
        let sphere_obj = BufReader::new(File::open("assets/sphere_mesh.obj")?);
        let sphere: Obj = load_obj(sphere_obj)?;

        Ok(MeshBuilder::new()
            .with_vertices(sphere.vertices)
            .into())
    }
}

impl SimpleState for MyState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        camera::init_camera(world);

        // what I should put here
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let assets_dir = app_root.join("assets");
    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([1.0, 1.0, 1.0, 1.0]),
                )
                .with_plugin(RenderShaded3D::default()),
        )?
        .with_bundle(TransformBundle::new())?;

    let mut game = Application::new(assets_dir, MyState, game_data)?;
    game.run();

    Ok(())
}
