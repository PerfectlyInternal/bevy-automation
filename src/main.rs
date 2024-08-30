use bevy::prelude::*;
use bevy_pancam::{PanCam, PanCamPlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod state;
mod asset;
mod ui;
mod item;
mod recipe;
mod machine;
mod inventory;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            WorldInspectorPlugin::new(),
            PanCamPlugin,
            asset::AssetPlugin,
            machine::MachinePlugin,
        ))
        .init_state::<state::AppState>()
        .add_systems(Startup, spawn_camera)
        .run();
}

fn spawn_camera(
    mut commands: Commands
) {
    commands.spawn(Camera2dBundle::default())
        .insert(PanCam::default());
}
