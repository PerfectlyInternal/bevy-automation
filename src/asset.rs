use std::collections::HashMap;

use bevy::prelude::*;
use bevy::asset::LoadedFolder;

use bevy_common_assets::ron::RonAssetPlugin;

use crate::state::AppState;
use crate::item::*;
use crate::recipe::*;

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            RonAssetPlugin::<ItemType>::new(&["item.ron"]),
            RonAssetPlugin::<Recipe>::new(&["recipe.ron"]),
        ));
        app.add_systems(OnEnter(AppState::LoadingAssets), load_assets);
        app.add_systems(OnEnter(AppState::InGame),
                (load_item_types, load_recipes)
            );
        app.add_systems(Update,
                check_assets.run_if(in_state(AppState::LoadingAssets)),
            );
    }
}

#[derive(Resource)]
struct AssetFolders {
    item_type_folder_handle: Handle<LoadedFolder>,
    recipe_folder_handle: Handle<LoadedFolder>,
}

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
