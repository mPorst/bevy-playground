use bevy::prelude::*;
use std::fmt;

// *** marker components *** //

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Asteroid;

#[derive(Component)]
pub struct Ant;

#[derive(Component)]
pub struct AntHive;

// *** inventory components *** //

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemType {
    IronOre,
    IronIngot,
    SteelPlate,
}

// Implement Display to get a human-readable name
impl fmt::Display for ItemType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            ItemType::IronOre => "Iron Ore",
            ItemType::IronIngot => "Iron Ingot",
            ItemType::SteelPlate => "Steel Plate",
        };
        write!(f, "{}", name)
    }
}

#[derive(Component)]
pub struct Storage {
    pub stored_ore: f32,
    pub max_stored_ore: f32,
}

#[derive(Component)]
pub struct PhysicalItem {
    pub item_type: ItemType,
    pub volume: f32,
    pub weight: f32,
}

#[derive(Component)]
pub struct InventoryItem {
    pub base_item: PhysicalItem,
    pub amount: f32,
}

impl InventoryItem {
    pub fn occupied_volume(&self) -> f32 {
        self.base_item.volume * self.amount
    }
}

#[derive(Component)]
pub struct Inventory {
    items: Vec<InventoryItem>,
    max_volume: f32,
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
        self.max_volume - self.occupied_volume()
    }

    pub fn add_item(&mut self, item: InventoryItem) -> Result<(), String> {
        if (item.occupied_volume() < self.free_volume()) {
            self.items.push(item);
            return Ok(());
        }
        return Err("Inventory full".to_string());
    }
}

#[derive(Component)]
pub struct Collidable;

#[derive(Component, Default, Clone, Copy)]
pub struct Target {
    pub target: Option<Entity>,
}

#[derive(Component, Default, Clone, Copy)]
pub struct HomeBase {
    pub home_base: Option<Entity>,
}
