use std::cmp::min;

use bevy::prelude::*;

use crate::item::*;

#[derive(Component)]
pub struct Inventory {
    pub stacks: Vec<ItemStack>,
    pub slots: u16,
}

impl Default for Inventory {
    fn default() -> Self {
        Inventory {
            stacks: Vec::<ItemStack>::new(),
            slots: 1,
        }
    }
}

impl Inventory {
    pub fn contains(&self, stacks: &Vec<ItemStack>) -> bool {
        for stack in stacks.iter() {
            let mut cont = true;
            for s in self.stacks.iter() {
                if s > stack {
                    cont = false;
                    break;
                }
            }
            if cont {
                return false;
            }
        }
        true
    }

    pub fn remove(&mut self, stacks: &Vec<ItemStack>) -> bool {
        if !self.contains(stacks) {
            return false;
        }
        for r in stacks.iter() {
            for s in self.stacks.iter_mut() {
                if r.item_type == s.item_type {
                    s.size -= r.size;
                    break;
                }
            }
        }
        true
    }

    pub fn used_full_slots(&self) -> u16 {
        let mut used_slots: u16 = 0;
        for stack in self.stacks.iter() {
            used_slots += stack.needed_full_slots();
        }
        used_slots
    }

    pub fn used_partial_slots(&self) -> u16 {
        let mut used_slots: u16 = 0;
        for stack in self.stacks.iter() {
            used_slots += stack.needed_partial_slots();
        }
        used_slots
    }

    pub fn can_fit(&self, stacks: &Vec<ItemStack>) -> bool {
        let mut needed_slots: u16 = 0;
        for stack in stacks.iter() {
            let mut has_type = false;
            for s in self.stacks.iter() {
                if s.item_type == stack.item_type {
                    has_type = true;
                    needed_slots += (s.clone() + stack.clone()).needed_slots();
                }
            }
            if !has_type {
                needed_slots += stack.needed_slots();
            }
        }
        needed_slots <= self.slots
    }

    pub fn add(&mut self, stacks: &Vec<ItemStack>) -> Vec<ItemStack> {
        let mut free_slots = self.slots - self.used_full_slots();
        let mut to_add = stacks.clone();
        for a in to_add.iter_mut() {
            let item_type = a.item_type.clone();
            let mut has_type = false;
            for s in self.stacks.iter_mut() {
                if item_type == s.item_type {
                    has_type = true;
                    if free_slots == 0 {
                        return to_add;
                    }
                    while free_slots > 0 && a.size > 0 {
                        let remainder = item_type.max_stack - (s.size % item_type.max_stack);
                        if remainder == item_type.max_stack {
                            // consume a new slot to add items
                            free_slots -= 1;
                        }
                        let amount_to_add = min(remainder, a.size);
                        s.size += amount_to_add;
                        a.size -= amount_to_add;
                    }
                    break;
                }
            }
            if !has_type {
                let needed_slots = min(free_slots, a.needed_slots());
                let size = min(a.size, needed_slots * item_type.max_stack);
                let stack = ItemStack { item_type, size };
                self.stacks.push(stack);
                a.size -= size;
                free_slots -= needed_slots;
                if free_slots == 0 {
                    break;
                }
            }
        }
        for i in (0..to_add.len()).rev() {
            if to_add[i].size == 0 {
                to_add.remove(i);
            }
        }
        to_add
    }

    pub fn add_strict(&mut self, stacks: &Vec<ItemStack>) -> bool {
        if self.can_fit(stacks) {
            let remainder = self.add(stacks);
            if !remainder.is_empty() {
                panic!("Thought stacks would fit in inventory, but they didnt! {} stacks were left.", remainder.len());
            }
            return true;
        }
        false
    }
}
