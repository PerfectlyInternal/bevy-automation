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

#[derive(Component, Default, PartialEq, Eq)]
pub enum MachineState {
    #[default]
    Idle,
    Crafting,
    Complete,
    InputShortage,
    OutputFull,
}

#[derive(Component, Default)]
pub struct CraftingTimer(Timer);

#[derive(Component, Default)]
pub struct InputInventory(pub Inventory);

#[derive(Component, Default)]
pub struct OutputInventory(pub Inventory);

#[derive(Bundle, Default)]
pub struct MachineBundle {
    input: InputInventory,
    output: OutputInventory,
    recipe: SetRecipe,
    state: MachineState,
    crafting_timer: CraftingTimer,
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
        let _ = inv.0.add(&[
            ItemStack { item_type: item_types.0.get(&1).unwrap().clone(), size: 10 }
        ]);
        println!("Spawned input stack!");
    }
}

fn start_crafts(
    mut q: Query<(&Machine, &SetRecipe, &mut MachineState, &mut CraftingTimer, &mut InputInventory)>,
) {
    for (machine, recipe_opt, mut state, mut timer, mut inv) in q.iter_mut() {
        if let Some(recipe) = &recipe_opt.0 {
            match *state { 
                MachineState::Complete => println!("Machine already has completed outputs, not starting recipe"),
                MachineState::Crafting => println!("Machine is already crafting!"),
                MachineState::InputShortage => println!("Machine has no inputs!"),
                MachineState::OutputFull => println!("Machine's output is full!"),
                MachineState::Idle => {
                    println!("Input contains {}", inv.0.stacks[0]);
                    if inv.0.remove(&recipe.inputs) {
                        timer.0 = Timer::from_seconds(recipe.duration / machine.0.crafting_speed, TimerMode::Once);
                        *state = MachineState::Crafting;
                        println!("Started crafting {}!", recipe.name);
                    } else {
                        *state = MachineState::InputShortage;
                        println!("Couldn't get items for {}", recipe.name);
                    }
                }
            }
        }
    }
}

fn update_crafting_state(
    time: Res<Time>,
    mut q: Query<(&SetRecipe, &mut MachineState, &mut CraftingTimer), With<Machine>>
) {
    for (recipe, mut state, mut timer) in q.iter_mut() {
        if recipe.0.is_some() && *state == MachineState::Crafting {
            timer.0.tick(time.delta());
            if timer.0.finished() {
                *state = MachineState::Complete;
                println!("Finished crafting {}!", recipe.0.as_ref().unwrap().name);
            }
        }
    }
}

fn spawn_craft_outputs(
    mut q: Query<(&SetRecipe, &mut MachineState, &mut OutputInventory), With<Machine>>
) {
    for (recipe_opt, mut state, mut inv) in q.iter_mut() {
        if *state == MachineState::Complete {
            if let Some(recipe) = &recipe_opt.0 {
                if inv.0.add_strict(&recipe.outputs) {
                    *state = MachineState::Idle;
                    println!("Spawned results of recipe {}!", recipe.name);
                    println!("Output now contains {}", inv.0.stacks[0]);
                } else {
                    *state = MachineState::OutputFull;
                    println!("Can't spawn recipe outputs, output is full!");
                }
            }
        }
    }
}
