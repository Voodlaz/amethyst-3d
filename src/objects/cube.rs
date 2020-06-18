use amethyst::prelude::*;
use amethyst::renderer::shape::Shape;
use amethyst::assets::AssetLoaderSystemData;

fn init_cube<M: amethyst::assets::Asset, P,N, Tangent, TextCoord>(world: &mut World) {
    let mesh = world.exec(|loader: AssetLoaderSystemData<'_, M>| {
        loader.load_from_data(
            Shape::Sphere(100, 100)
                .generate::<(Vec<P>, Vec<N>, Vec<Tangent>, Vec<TextCoord>)>(None)
                .into(),
            (),
        )
    });

    //let material_defaults = world.read_resource::<MaterialDefaults>().0.clone();
    //let material = world.exec(|loader: AssetLoaderSystemData<'_, Material>| {
    //    loader.load_from_data(
    //            Material {
    //                ..material_defaults
    //            },
    //            (),
    //        )
    //    },
    //);

    world.create_entity()
        .with(mesh)
        //.with(material)
        .build();
}
