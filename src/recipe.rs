use std::collections::HashMap;
use bevy::prelude::*;

#[derive(serde::Deserialize, Asset, TypePath, Clone)]
pub struct Recipe {
    pub name: String,
    pub id: u16,
    pub duration: f32,
    pub inputs: HashMap<u16, u16>,
    pub outputs: HashMap<u16, u16>
}

#[derive(Resource)]
pub struct RecipeList(pub HashMap<u16, Recipe>);
