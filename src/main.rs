use bevy::prelude::*;

pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;

fn main() {
    App::new()
    .insert_resource(ClearColor(Color::BLACK))
    .insert_resource(WindowDescriptor {
        width: WIDTH,
        height: HEIGHT,
        title: "Work in progress".to_string(),
        resizable: false,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .run();
}