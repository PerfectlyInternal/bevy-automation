use std::collections::HashMap;

use bevy:: {
    asset:: {
        AssetEvent,
        LoadedFolder,
    },
    reflect::TypePath,
    prelude::*,
};

use bevy_common_assets::ron::RonAssetPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            RonAssetPlugin::<ItemType>::new(&["item.ron"]),
            RonAssetPlugin::<Recipe>::new(&["recipe.ron"]),
        ))
        .init_state::<AppState>()
        .add_systems(OnEnter(AppState::LoadingAssets), load_assets)
        .add_systems(OnEnter(AppState::InGame), (
                load_item_types, load_recipes))
        .add_systems(Update, (
                check_assets.run_if(in_state(AppState::LoadingAssets)),
            ))
        .run();
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    LoadingAssets,
    InGame,
    Finished,
}

#[derive(serde::Deserialize, Asset, TypePath, Clone)]
struct ItemType {
    name: String,
    id: u16,
    max_stack: u16,
}

#[derive(serde::Deserialize, Asset, TypePath, Clone)]
struct Recipe {
    name: String,
    id: u16,
    inputs: HashMap<u16, u16>,
    outputs: HashMap<u16, u16>
}

#[derive(Resource)]
struct AssetFolders {
    item_type_folder_handle: Handle<LoadedFolder>,
    recipe_folder_handle: Handle<LoadedFolder>,
}

#[derive(Resource)]
struct ItemTypeList(pub HashMap<u16, ItemType>);

#[derive(Resource)]
struct RecipeList(pub HashMap<u16, Recipe>);

fn load_assets(
    mut commands: Commands,
    server: Res<AssetServer>,
) {
    commands.insert_resource(
        AssetFolders {
            item_type_folder_handle: server.load_folder("items"),
            recipe_folder_handle: server.load_folder("recipes"),
        }
    );
}

fn check_assets(
    mut app_next_state: ResMut<NextState<AppState>>,
    mut events: EventReader<AssetEvent<LoadedFolder>>,
) {
    for event in events.read() {
        if let AssetEvent::LoadedWithDependencies { id: _id } = event {
            println!("Asset folder loaded!");
            app_next_state.set(AppState::InGame);
        }
    }
}

fn load_item_types(
    mut commands: Commands,
    item_types: Res<Assets<ItemType>>,
) {
    let mut type_list = ItemTypeList(HashMap::<u16, ItemType>::new());
    for (id, item_type) in item_types.iter() {
        println!("{}, name: {}, id: {} max stack: {}", id, item_type.name, item_type.id, item_type.max_stack);
        type_list.0.insert(item_type.id, item_type.clone());
    } 

    commands.insert_resource(type_list)
}

fn load_recipes(
    mut commands: Commands,
    recipes: Res<Assets<Recipe>>,
) {
    let mut recipe_list = RecipeList(HashMap::<u16, Recipe>::new());
    for (id, recipe) in recipes.iter() {
        println!("{}, name: {}, id: {}", id, recipe.name, recipe.id);
        recipe_list.0.insert(recipe.id, recipe.clone());
    } 

    commands.insert_resource(recipe_list)
}
