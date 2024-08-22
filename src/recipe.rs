use std::collections::HashMap;
use bevy::prelude::*;

use crate::item::*;

#[derive(serde::Deserialize, Asset, TypePath, Clone)]
pub struct RecipeTemplate {
    pub name: String,
    pub id: u16,
    pub duration: f32,
    pub inputs: HashMap<u16, u16>,
    pub outputs: HashMap<u16, u16>
}

#[derive(Clone)]
pub struct Recipe {
    pub name: String,
    pub id: u16,
    pub duration: f32,
    pub inputs: Vec<ItemStack>,
    pub outputs: Vec<ItemStack>
}

impl Recipe {
    pub fn from_template(
        template: &RecipeTemplate,
        item_types: &ItemTypeList
    ) -> Recipe {
        let mut inputs = Vec::<ItemStack>::new();
        let mut outputs = Vec::<ItemStack>::new();
        for (type_id, amount) in template.inputs.iter() {
            inputs.push(
                ItemStack {
                    item_type: item_types.0.get(&type_id).unwrap().clone(),
                    size: *amount
                }
            );
        }
        for (type_id, amount) in template.outputs.iter() {
            outputs.push(
                ItemStack {
                    item_type: item_types.0.get(&type_id).unwrap().clone(),
                    size: *amount
                }
            );
        }
        Recipe {
            name: template.name.clone(),
            id: template.id,
            duration: template.duration,
            inputs,
            outputs,
        }
    }
}

#[derive(Resource)]
pub struct RecipeList(pub HashMap<u16, Recipe>);
