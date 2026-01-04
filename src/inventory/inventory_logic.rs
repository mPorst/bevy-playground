use super::inventory_components::*;
use crate::common::common_items::*;

pub fn does_stack_fit(into: &Inventory, item: &InventoryItemStack) -> bool {
    if (into.free_volume() >= item.occupied_volume()) {
        return true;
    }
    return false;
}

pub fn max_fitting_amount(into: &Inventory, item: &InventoryItemStack) -> u32 {
    let all_available_space = (into.free_volume() / item.base_item.volume).trunc() as u32;
    // it could be that more than the stack size fits into the inventory
    std::cmp::min(item.amount, all_available_space)
}

#[cfg(test)]
mod tests {
    use crate::common::common_items::types::{InventoryItem, ItemType};

    use super::*;

    struct TestContext {
        inv: Inventory,
        item: InventoryItem,
        stack: InventoryItemStack,
    }

    fn setup() -> TestContext {
        let invItem = InventoryItem {
            item_type: ItemType::IronIngot,
            volume: 10.0,
            weight: 50.0,
        };
        TestContext {
            inv: Inventory {
                items: vec![],
                volume_capacity: 100.0,
            },
            item: invItem.clone(),
            stack: InventoryItemStack {
                base_item: invItem.clone(),
                amount: 10,
            },
        }
    }

    #[test]
    // stack fits into inv
    fn test_fit_fitting() {
        let context = setup();

        assert_eq!(does_stack_fit(&context.inv, &context.stack), true);
    }

    #[test]
    // stack is made too large to fit into inv
    fn test_fit_not_fitting_1() {
        let mut context = setup();
        context.stack.amount = 11;

        assert_eq!(does_stack_fit(&context.inv, &context.stack), false);
    }

    #[test]
    // inv was reduced in size
    fn test_fit_not_fitting_2() {
        let mut context = setup();
        context.inv.volume_capacity = 99.99;

        assert_eq!(does_stack_fit(&context.inv, &context.stack), false);
    }

    #[test]
    // standard case: 100.0 volume avail and 10.0 volume per iron ingot
    fn test_max_fit_ok() {
        let mut context = setup();

        assert_eq!(max_fitting_amount(&context.inv, &context.stack), 10);
    }

    #[test]
    // inventory can hold more than the stack size
    fn test_max_fit_larger() {
        let mut context = setup();
        context.inv.volume_capacity = 200.0;

        // since max_fitting_amount explicitly acts on an item stack it should NOT return more than the item stack size
        assert_eq!(max_fitting_amount(&context.inv, &context.stack), 10);
    }

    #[test]
    // inventory can hold less than the stack size
    fn test_max_fit_smaller() {
        let mut context = setup();
        context.inv.volume_capacity = 59.789; // this makes sure only truncate is used, not round.

        assert_eq!(max_fitting_amount(&context.inv, &context.stack), 5);
    }
}
