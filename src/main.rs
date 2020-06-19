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
use std::io::Cursor;
use obj::{load_obj, Obj};

mod objects;
use crate::objects::{camera, sphere};

struct MyState;

#[derive(Clone, Debug)]
struct ObjMesh;

impl AssetFormat<MeshData> for ObjMesh {
    fn name(&self) -> &'static str {
        "OBJ"
    }

    /// Reads the given bytes and produces asset data.
    fn import_simple(&self, bytes: Vec<u8>) -> Result<MeshData, Error> {
        let input = Cursor::new(bytes);
        let sphere: Obj = load_obj(input)?;

        let capacity = sphere.vertices.len() * 3;
        let mut pos = Vec::with_capacity(capacity);
        let mut norm = Vec::with_capacity(capacity);

        for vertex in sphere.vertices {
            pos.push(Position([
                vertex.position[0],
                vertex.position[1],
                vertex.position[2],
            ]));

            norm.push(Normal([
                vertex.normal[0],
                vertex.normal[1],
                vertex.normal[2]
            ]))
        }

        Ok(MeshBuilder::new()
            .with_vertices(pos)
            .with_vertices(norm)
            .into())
    }
}

impl SimpleState for MyState {
    fn on_start(&mut self,data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;
        world.insert(0usize);

        camera::init_camera(world);

        // Add custom cube object to scene
        let (mesh, mtl) = {
            let mat_defaults = world.read_resource::<MaterialDefaults>();
            let loader = world.read_resource::<Loader>();

            let meshes = &world.read_resource();
            let textures = &world.read_resource();
            let materials = &world.read_resource();

            let mesh: Handle<Mesh> = loader.load("sphere_mesh.obj", ObjMesh, (), meshes);
            let albedo = loader.load_from_data(
                load_from_srgba(Srgba::new(0.1, 0.5, 0.3, 1.0)).into(),
                (),
                textures,
            );
            let mat: Handle<Material> = loader.load_from_data(
                Material {
                    albedo,
                    ..mat_defaults.0.clone()
                },
                (),
                materials,
            );

            (mesh, mat)
        };

        let mut trans = Transform::default();
        trans.set_translation_xyz(-5.0, 0.0, 0.0);
        trans.set_scale(Vector3::new(2.0, 2.0, 2.0));
        world
            .create_entity()
            .with(mesh)
            .with(mtl)
            .with(trans)
            .build();
    }
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
