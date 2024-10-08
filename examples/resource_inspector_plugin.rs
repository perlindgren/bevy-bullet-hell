use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;

// `InspectorOptions` are completely optional
#[derive(Reflect, Resource, Default, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
struct Configuration {
    name: String,

    option: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Configuration>() // `ResourceInspectorPlugin` won't initialize the resource
        .register_type::<Configuration>() // you need to register your type to display it
        .add_plugins(ResourceInspectorPlugin::<Configuration>::default())
        // also works with built-in resources, as long as they are `Reflect`
        .add_plugins(ResourceInspectorPlugin::<Time>::default())
        .insert_resource(Configuration {
            name: "Hi".to_string(),
            option: -0.5,
        })
        .run();
}
