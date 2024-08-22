use std::collections::HashMap;

use bevy::prelude::*;
use bevy::asset::LoadedFolder;

use bevy_common_assets::ron::RonAssetPlugin;

use crate::state::AppState;
use crate::item::*;
use crate::recipe::*;
use crate::machine::*;

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            RonAssetPlugin::<ItemType>::new(&["item.ron"]),
            RonAssetPlugin::<RecipeTemplate>::new(&["recipe.ron"]),
            RonAssetPlugin::<MachineTemplate>::new(&["machine.ron"]),
        ));
        app.add_systems(
            OnEnter(AppState::LoadingAssetFolders),
            load_asset_folders);
        app.add_systems(
            Update,
            check_asset_folders.run_if(in_state(AppState::LoadingAssetFolders)),
        );
        app.add_systems(
            OnEnter(AppState::LoadingAssets),
            (load_item_types, load_recipes, load_machines).chain()
        );
        app.add_systems(
            Update,
            start_game.run_if(
                    in_state(AppState::LoadingAssets)
                    .and_then(resource_exists::<ItemTypeList>)
                    .and_then(resource_exists::<RecipeList>)
                    .and_then(resource_exists::<MachineList>)
            )
        );
    }
}

#[allow(dead_code)]
#[derive(Resource)]
struct AssetFolders {
    item_type_folder_handle: Handle<LoadedFolder>,
    recipe_folder_handle: Handle<LoadedFolder>,
    machine_folder_handle: Handle<LoadedFolder>,
}
#[warn(dead_code)]

fn load_asset_folders(
    mut commands: Commands,
    server: Res<AssetServer>,
) {
    commands.insert_resource(
        AssetFolders {
            item_type_folder_handle: server.load_folder("items"),
            recipe_folder_handle: server.load_folder("recipes"),
            machine_folder_handle: server.load_folder("machines"),
        }
    );
}

fn check_asset_folders(
    mut app_next_state: ResMut<NextState<AppState>>,
    mut events: EventReader<AssetEvent<LoadedFolder>>,
) {
    for event in events.read() {
        if let AssetEvent::LoadedWithDependencies { id: _id } = event {
            println!("Asset folder loaded!");
            app_next_state.set(AppState::LoadingAssets);
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
    recipe_templates: Res<Assets<RecipeTemplate>>,
    item_types: Res<ItemTypeList>,
) {
    let mut recipe_list = RecipeList(HashMap::<u16, Recipe>::new());
    for (id, template) in recipe_templates.iter() {
        println!("{}, name: {}, id: {}", id, template.name, template.id);
        let new_recipe = Recipe::from_template(template, &item_types);
        recipe_list.0.insert(template.id, new_recipe);
    } 

    commands.insert_resource(recipe_list)
}

fn load_machines(
    mut commands: Commands,
    machines: Res<Assets<MachineTemplate>>,
) {
    let mut machine_list = MachineList(HashMap::<u16, MachineTemplate>::new());
    for (id, machine) in machines.iter() {
        println!("{}, name: {}, id: {}, crafting speed: {}", id, machine.name, machine.id, machine.crafting_speed);
        machine_list.0.insert(machine.id, machine.clone());
    } 

    commands.insert_resource(machine_list)
}

fn start_game(
    mut app_next_state: ResMut<NextState<AppState>>,
) {
    println!("Starting game!");
    app_next_state.set(AppState::InGame);
}
