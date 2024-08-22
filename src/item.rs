use std::ops::{Sub, Add};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Display;

use bevy::prelude::*;

#[derive(serde::Deserialize, Asset, TypePath, Clone, Hash)]
pub struct ItemType {
    pub name: String,
    pub id: u16,
    pub max_stack: u16,
}

impl PartialEq for ItemType {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for ItemType {}

#[derive(Resource)]
pub struct ItemTypeList(pub HashMap<u16, ItemType>);

#[derive(Clone)]
pub struct ItemStack {
    pub item_type: ItemType,
    pub size: u16,
}

impl Display for ItemStack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{} {}(s)", self.size, self.item_type.max_stack, self.item_type.name)
    }
}

impl PartialEq for ItemStack {
    fn eq(&self, other: &Self) -> bool {
        self.item_type == other.item_type && self.size == other.size
    }
}

impl Eq for ItemStack {}

impl PartialOrd for ItemStack {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.item_type != other.item_type {
            debug!("Cannot compare item stacks with differing types!");
            return None;
        }
        Some(self.size.cmp(&other.size))
    }
}

impl Sub for ItemStack {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        if !(self >= rhs) {
            panic!("Cannot subtract item stacks with different types, or with a negative size result!");
        }
        Self {
            item_type: self.item_type,
            size: self.size - rhs.size,
        }
    }
}

impl Add for ItemStack {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.item_type != rhs.item_type {
            panic!("Cannot add item stacks with different types!");
        }
        Self {
            item_type: self.item_type,
            size: self.size + rhs.size,
        }
    }
}

impl ItemStack {
    pub fn needed_full_slots(&self) -> u16 {
        self.size / self.item_type.max_stack
    }

    pub fn needed_partial_slots(&self) -> u16 {
        if self.size % self.item_type.max_stack > 0 { 1 } else { 0 } 
    }

    pub fn needed_slots(&self) -> u16 {
        self.needed_full_slots() + self.needed_partial_slots()
    }
}
