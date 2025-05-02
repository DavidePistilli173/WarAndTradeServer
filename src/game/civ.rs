use crate::game::res::{Recipe, Resource, ResourceId};
use crate::game::storage::Storage;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Amount of metric tonnes of food needed by one person for one day.
const PERSON_EAT_NEEDS: f64 = 0.001;
/// Amount of metric tonnes of drinkable resources needed by one person for one day.
const PERSON_DRINK_NEEDS: f64 = 0.002;

/// Civilisation data.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Civ {
    /// Name of the civilisation.
    civ_name: String,
    /// Total population.
    population: u64,
    /// Fraction of the food needs that are satisfied. [0.0, 1.0]
    satisfied_food_needs: f64,
    /// Fraction of the drink needs that are satisfied. [0.0, 1.0]
    satisfied_drink_needs: f64,
    /// Civilisation storage.
    storage: Storage,
}

impl Civ {
    pub fn civ_name(&self) -> &str {
        &self.civ_name
    }

    pub fn new(name: String) -> Self {
        Self {
            civ_name: name,
            population: 1000,
            satisfied_food_needs: 1.0,
            satisfied_drink_needs: 1.0,
            storage: Storage::new(),
        }
    }

    pub fn population(&self) -> u64 {
        self.population
    }

    /// Get the fraction of food and drink needs that are satisfied.
    pub fn satisfied_needs(&self) -> (f64, f64) {
        (self.satisfied_food_needs, self.satisfied_drink_needs)
    }

    /// Simulate a single day.
    pub fn simulate(&mut self) {
        // Compute the current population needs.
        let edible_needs = PERSON_EAT_NEEDS * (self.population as f64);
        let drink_needs = PERSON_DRINK_NEEDS * (self.population as f64);

        // Consume food and drink.
        (self.satisfied_food_needs, self.satisfied_drink_needs) = self
            .storage
            .subtract_population_needs(edible_needs, drink_needs);

        // Compute population loss.
        let mut food_loss_perc = 0.0;
        if self.satisfied_food_needs < 1.0 {
            food_loss_perc = (1.0 - self.satisfied_food_needs) * 0.1;
        }

        let mut drink_loss_perc = 0.0;
        if self.satisfied_drink_needs < 1.0 {
            drink_loss_perc = (1.0 - self.satisfied_drink_needs) * 0.1;
        }

        self.population -= f64::ceil(self.population as f64 * food_loss_perc) as u64;
        self.population -= f64::ceil(self.population as f64 * drink_loss_perc) as u64;
    }
}
