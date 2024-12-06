use bevy::prelude::*;
use bevy::window::{Window, WindowPlugin, PrimaryWindow};
use bevy_egui::EguiPlugin;
use bevy_prototype_lyon::prelude::*;

mod components;
mod resources;
mod systems;

use resources::*;
use systems::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "ALFA Particle Life".to_string(),
                    resolution: (1280.0, 720.0).into(),
                    ..default()
                }),
                ..default()
            })
        )
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(GlobalParameters::default())
        .insert_resource(Population::new(4, 5))
        .insert_resource(SpeciesRadii::new(4))
        .insert_resource(AdaptiveLearningState::default())
        .insert_resource(CurrentGenomeIndex(0))
        .insert_resource(InteractionRules::default())
        // Use add_plugin for single plugins like EguiPlugin:
        .add_plugin(EguiPlugin)
        // Add the ShapePlugin so that shapes are rendered:
        .add_plugin(ShapePlugin)
        // Setup systems
        .add_systems(Startup, setup_system)
        .add_systems(Update, physics_system)
        .add_systems(Update, boundary_system)
        .add_systems(Update, adaptive_learning_system)
        .add_systems(Update, ui_system)
        .run();
}
