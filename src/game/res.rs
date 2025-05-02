use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

/// Available resources.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
pub enum ResourceId {
    DirtyWater,
}

/// A gatherable, consumable resource.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Resource {
    /// Unique resource ID.
    pub id: ResourceId,
    /// Resource amount, specified in metric tonnes (1000Kg).
    pub amount: f64,
    /// If true, this resource can be eaten by the population.
    pub edible: bool,
    /// If true, this resource can be drunk by the population.
    pub drinkable: bool,
}

/// Recipe for converting a given set of resources into another set of resources.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Recipe<const IN_SIZE: usize, const OUT_SIZE: usize> {
    #[serde(with = "BigArray")]
    pub inputs: [Resource; IN_SIZE],
    #[serde(with = "BigArray")]
    pub outputs: [Resource; OUT_SIZE],
}
