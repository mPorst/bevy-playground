use std::fmt;

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

#[derive(Clone)]
pub struct InventoryItem {
    pub item_type: ItemType,
    pub volume: f32,
    pub weight: f32,
}
