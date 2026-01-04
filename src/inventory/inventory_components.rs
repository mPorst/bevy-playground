use super::inventory_logic::*;
use crate::common::common_items::types::*;

use bevy::prelude::*;

#[derive(Clone)]
pub struct InventoryItemStack {
    pub base_item: InventoryItem,
    pub amount: i32,
}

impl InventoryItemStack {
    pub fn occupied_volume(&self) -> f32 {
        self.base_item.volume * self.amount as f32
    }
}

#[derive(Component)]
pub struct Inventory {
    pub items: Vec<InventoryItemStack>,
    pub volume_capacity: f32,
}

impl Inventory {
    pub fn occupied_volume(&self) -> f32 {
        let mut volume = 0.0;
        for item in self.items.iter() {
            volume += item.occupied_volume();
        }
        volume
    }

    pub fn free_volume(&self) -> f32 {
        self.volume_capacity - self.occupied_volume()
    }

    pub fn add_item(&mut self, item: InventoryItemStack) -> Result<(), String> {
        if item.occupied_volume() < self.free_volume() {
            self.items.push(item);
            return Ok(());
        }
        return Err("Inventory full".to_string());
    }

    // transfer items INTO self
    pub fn transfer_item_into(&mut self, from: &mut Inventory, items: InventoryItemStack) {}

    // transfer items FROM self
    pub fn transfer_item_from(&mut self, to: &mut Inventory, items: InventoryItemStack) {}
}
