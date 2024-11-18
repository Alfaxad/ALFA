use alfa::{
    bee::{BeePlugin},
    gui::{GuiPlugin, SimSettings},
    pathviz::PathVizPlugin,
    pheromone::PheromonePlugin,
    *,
};
use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use bevy_pancam::{PanCam, PanCamPlugin};

#[derive(Component)]
struct FollowCamera;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resizable: false,
                        focused: true,
                        resolution: (1920.0, 1080.0).into(),
                        title: "ALFA - Bee Colony Simulation".to_string(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        // External plugins & systems
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_systems(Update, bevy::window::close_on_esc)
        .add_plugins(PanCamPlugin)
        // Default Resources
        .insert_resource(ClearColor(Color::BLACK)) // Set background to dark/black
        .insert_resource(Msaa::Off)
        // Systems
        .add_systems(Startup, setup)
        .add_systems(Update, bee_follow_camera)
        // Internal Plugins
        .add_plugins(BeePlugin)          // Bee behavior
        .add_plugins(PheromonePlugin)    // Pheromone dynamics
        .add_plugins(PathVizPlugin)      // Path visualization
        .add_plugins(GuiPlugin)          // GUI
        .run();
}

fn bee_follow_camera(
    bee_pos: Res<BeeFollowCameraPos>,
    sim_settings: Res<SimSettings>,
    mut camera_query: Query<&mut Transform, With<FollowCamera>>,
) {
    if !sim_settings.is_camera_follow {
        return;
    }

    let mut transform = camera_query.single_mut();
    transform.translation = Vec3::new(bee_pos.0.x, bee_pos.0.y, 3.0);
}

fn setup(mut commands: Commands) {
    commands
        .spawn((
            Camera2dBundle {
                camera: Camera {
                    hdr: true,
                    ..default()
                },
                tonemapping: Tonemapping::TonyMcMapface,
                ..default()
            },
            BloomSettings::default(),
            FollowCamera,
        ))
        .insert(PanCam::default());

    // Bee hive sprite
    commands.spawn(SpriteBundle {
        texture: asset_server.load("assets/hive.png"),
        sprite: Sprite {
            color: Color::rgb(1.5, 1.5, 1.5),
            ..default()
        },
        transform: Transform::from_xyz(750.0, -350.0, 2.0)
            .with_scale(Vec3::splat(2.5)),
        ..Default::default()
    });

    // Flower sprite
    commands.spawn(SpriteBundle {
        texture: asset_server.load("assets/flowers.png"),
        sprite: Sprite {
            color: Color::rgb(1.5, 1.5, 1.5),
            ..default()
        },
        transform: Transform::from_xyz(-750.0, 400.0, 2.0)
            .with_scale(Vec3::splat(2.0)),
        ..Default::default()
    });
}
