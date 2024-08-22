pub struct ItemSet {
    items: HashMap<ItemType, u16>,
    slots: u16,
}

impl Default for ItemSet {
    fn default() -> Self {
        Self {
            items: HashMap::<ItemType, u16>::new(),
            slots: 1,
        }
    }
}

impl ItemSet {
    pub fn from_item_stacks(stacks: &Vec<ItemStack>) -> Self {
        let mut new = ItemSet::default();
        for stack in stacks.iter() {
            if let Some(amount) = new.items.get(&stack.item_type) {
                new.items.insert(stack.item_type.clone(), stack.size + amount);
            } else {
                new.items.insert(stack.item_type.clone(), stack.size);
            }
        }
        new.slots = new.used_slots();
        new
    }

    pub fn from_hashmap(map: HashMap<ItemType, u16>) -> Self {
        let mut new = ItemSet { items: map, slots: 0 };
        new.slots = new.used_slots();
        new
    }

    pub fn used_slots(&self) -> u16 {
        let mut used_slots: u16 = 0;
        for (item_type, amount) in self.items.iter() {
            used_slots += amount / item_type.max_stack;
            if amount % item_type.max_stack > 0 {
                used_slots += 1;
            }
        }
        used_slots
    }

    pub fn free_slots(&self) -> u16 {
        self.slots - self.used_slots()
    }

    // add other to self, return leftovers
    pub fn add(&self, other: &Self) -> Option<Self> {
        let mut free_slots = self.free_slots();
        let mut to_insert = other.items.clone();
        for (item_type, amount) in to_insert.iter() {
            if free_slots == 0 {
                break;
            }
        }
        if to_insert.len() == 0 {
            return None;
        }
        Some(Self::from_hashmap(to_insert))
    }

    pub fn contains(&self, other: &Self) -> bool {
        for (item_type, amount) in other.items.iter() {
            if let Some(self_amount) = self.items.get(item_type) {
                if self_amount < amount {
                    return false;
                }
            } else {
                return false; // this set doesnt contain this item type
            }
        }
        true
    }

    pub fn remove(&mut self, other: &Self) -> bool {
        if !self.contains(other) {
            return false;
        }
        for (item_type, amount) in other.items.iter() {
            let new_amount = self.items.get(item_type).unwrap() - amount;
            if new_amount > 0 {
                self.items.insert(item_type.clone(), new_amount);
            } else {
                self.items.remove(item_type);
            }
        }
        true
    }
}
