use bevy::prelude::*;

// Gameboy resolution is 160x144 but this should be upscaled
// 160x144(1x), 640x576(4x), 800x720(5x), 960x864(6x), 1120x1008(7x), 1280x1152(8x)
pub const WIDTH: f32 = 640.0;
pub const HEIGHT: f32 = 576.0;

fn main() {
    App::new()
    .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
    .insert_resource(WindowDescriptor {
        width: WIDTH,
        height: HEIGHT,
        title: "Cabinned".to_string(),
        resizable: false,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    // .add_startup_system(setup)
    .add_startup_system(spawn_camera)
    .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands.spawn_bundle(Camera2dBundle {
        camera_2d: Camera2d {
            ..default()
        },
        camera: Camera {
            priority: 1,
            ..default()
        },
        ..default()
    });
}