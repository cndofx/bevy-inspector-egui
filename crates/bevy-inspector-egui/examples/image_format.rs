use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use bevy_inspector_egui::{
    bevy_egui::EguiPlugin, prelude::*, quick::ResourceInspectorPlugin, DefaultInspectorConfigPlugin,
};

#[derive(Resource, Reflect, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct Configuration {
    number: u8,
    // image to display
    image: Handle<Image>,
}

impl FromWorld for Configuration {
    fn from_world(world: &mut World) -> Self {     

        let mut images = world.resource_mut::<Assets<Image>>();
        let image = Image::new_fill(
            Extent3d {
                width: 255,
                height: 255,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            &[0, 0, 0, 255],
            TextureFormat::Rgba8Unorm, // didnt work before, does with patch
            //TextureFormat::Rgba8UnormSrgb // always works
        );
        let image_handle = images.add(image);

        Configuration {
            number: 0,
            image: image_handle,
        }
    }
}

fn main() {
    let mut app = App::new();

    // common setup
    app.add_plugins(DefaultPlugins)
        .add_plugins((EguiPlugin, DefaultInspectorConfigPlugin))
        .init_resource::<Configuration>()
        .register_type::<Configuration>()
        .add_plugins(ResourceInspectorPlugin::<Configuration>::default())
        .run();
}
