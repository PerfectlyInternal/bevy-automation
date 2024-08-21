use std::collections::HashMap;
use bevy::prelude::*;

#[derive(serde::Deserialize, Asset, TypePath, Clone)]
pub struct ItemType {
    pub name: String,
    pub id: u16,
    pub max_stack: u16,
}

#[derive(Resource)]
pub struct ItemTypeList(pub HashMap<u16, ItemType>);
