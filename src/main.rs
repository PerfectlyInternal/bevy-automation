use bevy::prelude::*;

mod state;
mod asset;
mod item;
mod recipe;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            asset::AssetPlugin,
        ))
        .init_state::<state::AppState>()
        .run();
}
