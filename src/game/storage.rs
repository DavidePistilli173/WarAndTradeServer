use crate::game::res::{Resource, ResourceId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Storage {
    /// Edible resources.
    edible: HashMap<ResourceId, Resource>,
    /// Total amount of stored edible resources. [T]
    edible_amount: f64,
    /// Drinkable resources.
    drinkable: HashMap<ResourceId, Resource>,
    /// Total amount of stored drinkable resources. [T]
    drinkable_amount: f64,
    /// Generic resources.
    generic: HashMap<ResourceId, Resource>,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            edible: HashMap::new(),
            edible_amount: 0.0,
            drinkable: HashMap::new(),
            drinkable_amount: 0.0,
            generic: HashMap::new(),
        }
    }

    /// Subtract the given food and drink needs from storage and return the percentages of needs that were satisfied.
    pub fn subtract_population_needs(&mut self, food_needs: f64, drink_needs: f64) -> (f64, f64) {
        let mut remaining_food_needs = food_needs;
        let mut remaining_drink_needs = drink_needs;

        for res in self.edible.iter_mut() {
            if remaining_food_needs == 0.0 {
                break;
            }

            let consume_amount = f64::min(remaining_food_needs, res.1.amount);
            res.1.amount -= consume_amount;
            self.edible_amount -= consume_amount;
            remaining_food_needs -= consume_amount;
        }

        for res in self.drinkable.iter_mut() {
            if remaining_drink_needs == 0.0 {
                break;
            }

            let consume_amount = f64::min(remaining_drink_needs, res.1.amount);
            res.1.amount -= consume_amount;
            self.drinkable_amount -= consume_amount;
            remaining_drink_needs -= consume_amount;
        }

        let mut food_percentage = 1.0;
        if food_needs != 0.0 {
            food_percentage = 1.0 - remaining_food_needs / food_needs;
        }

        let mut drink_percentage = 1.0;
        if drink_needs != 0.0 {
            drink_percentage = 1.0 - remaining_drink_needs / drink_needs;
        }

        (food_percentage, drink_percentage)
    }
}
