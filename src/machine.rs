use std::collections::HashMap;
use bevy::prelude::*;

use crate::state::*;
use crate::item::*;
use crate::recipe::*;
use crate::inventory::*;

pub struct MachinePlugin;

impl Plugin for MachinePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::InGame),
            (spawn_machine, set_recipe, spawn_inputs).chain());
        app.add_systems(
            Update,
            (start_crafts, update_crafting_state, spawn_craft_outputs)
        );
    }
}

#[derive(serde::Deserialize, Asset, TypePath, Clone)]
pub struct MachineTemplate {
    pub name: String,
    pub id: u16,
    pub crafting_speed: f32,
    pub valid_recipes: Vec<u16>
}

#[derive(Resource)]
pub struct MachineList(pub HashMap<u16, MachineTemplate>);

#[derive(Component)]
pub struct Machine(pub MachineTemplate);

#[derive(Component, Default)]
pub struct SetRecipe(pub Option<Recipe>);

#[derive(Component, Default)]
pub struct CraftingState {
    pub crafting: bool, // is currently crafting?
    pub complete: bool, // do we need to spawn outputs?
    pub timer: Timer,
}

#[derive(Component, Default)]
pub struct InputInventory(pub Inventory);

#[derive(Component, Default)]
pub struct OutputInventory(pub Inventory);

#[derive(Bundle, Default)]
pub struct MachineBundle {
    input: InputInventory,
    output: OutputInventory,
    recipe: SetRecipe,
    crafting_state: CraftingState,
}

fn spawn_machine(
    mut commands: Commands,
    machine_list: Res<MachineList>,
) {
    commands.spawn((
        Machine(machine_list.0.get(&1).unwrap().clone()),
        MachineBundle::default(),
    ));
    println!("Spawned machine!");
}

fn set_recipe(
    mut q: Query<&mut SetRecipe, With<Machine>>,
    recipe_list: Res<RecipeList>,
) {
    for mut recipe in q.iter_mut() {
        recipe.0 = Some(recipe_list.0.get(&1).unwrap().clone());
        println!("Set recipe to {}", recipe.0.as_ref().unwrap().name);
    }
}

fn spawn_inputs(
    mut q: Query<&mut InputInventory, With<Machine>>,
    item_types: Res<ItemTypeList>,
) {
    for mut inv in q.iter_mut() {
        let rem = inv.0.add(&vec![
            ItemStack { item_type: item_types.0.get(&1).unwrap().clone(), size: 10 }
        ]);
        if !rem.is_empty() {
            println!("{} left after adding inputs", rem[0]);
        } else {
            println!("Spawned input stack!");
        }
    }
}

fn start_crafts(
    mut q: Query<(&Machine, &SetRecipe, &mut CraftingState, &mut InputInventory)>
) {
    for (machine, recipe_opt, mut state, mut inv) in q.iter_mut() {
        if let Some(recipe) = &recipe_opt.0 {
            if state.complete { 
                println!("Machine already has completed outputs, not starting recipe");
                continue;
            }
            if state.crafting {
                println!("Machine is already crafting");
                continue;
            }
            println!("Input contains {}", inv.0.stacks[0]);
            if inv.0.remove(&recipe.inputs) {
                state.timer = Timer::from_seconds(recipe.duration / machine.0.crafting_speed, TimerMode::Once);
                state.crafting = true;
                println!("Started crafting {}!", recipe.name);
            } else {
                println!("Couldn't get items for {}", recipe.name);
            }
        }
    }
}

fn update_crafting_state(
    time: Res<Time>,
    mut q: Query<(&SetRecipe, &mut CraftingState), With<Machine>>
) {
    for (recipe, mut state) in q.iter_mut() {
        if let Some(_) = recipe.0 {
            if state.crafting {
                state.timer.tick(time.delta());
                if state.timer.finished() {
                    state.crafting = false;
                    state.complete = true;
                    println!("Finished crafting {}!", recipe.0.as_ref().unwrap().name);
                }
            }
        }
    }
}

fn spawn_craft_outputs(
    mut q: Query<(&SetRecipe, &mut CraftingState, &mut OutputInventory), With<Machine>>
) {
    for (recipe_opt, mut state, mut inv) in q.iter_mut() {
        if state.complete {
            if let Some(recipe) = &recipe_opt.0 {
                if inv.0.add_strict(&recipe.outputs) {
                    state.complete = false;
                    println!("Spawned results of recipe {}!", recipe.name);
                    println!("Output now contains {}", inv.0.stacks[0]);
                }
            }
        }
    }
}
