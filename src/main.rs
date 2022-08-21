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
    .add_startup_system(setup)
    .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("tileset-experimentation.png"),
        ..default()
    });
}