use bevy::prelude::*;

mod state;
mod asset;
mod item;
mod recipe;
mod machine;
mod inventory;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            asset::AssetPlugin,
            machine::MachinePlugin,
        ))
        .init_state::<state::AppState>()
        .run();
}
