use super::inventory_components::*;
use crate::common::common_items::*;

pub fn does_stack_fit(into: &Inventory, item: &InventoryItemStack) -> bool {
    if (into.free_volume() >= item.occupied_volume()) {
        return true;
    }
    return false;
}

//pub fn max_fitting_amount(into: &Inventory, item: &InventoryItemStack) -> f32

#[cfg(test)]
mod tests {
    use crate::common::common_items::types::{InventoryItem, ItemType};

    use super::*;

    #[test]
    fn test_fit_fitting() {
        let inv = Inventory {
            items: vec![],
            volume_capacity: 100.0,
        };
        let item = InventoryItem {
            item_type: ItemType::IronIngot,
            volume: 10.0,
            weight: 50.0,
        };
        let stack = InventoryItemStack {
            base_item: item,
            amount: 10,
        };

        assert_eq!(does_stack_fit(&inv, &stack), true);
    }
}
