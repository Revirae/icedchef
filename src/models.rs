use uuid::Uuid;
use serde::{Deserialize, Serialize};
use crate::states::InventoryState;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Inventory {
    #[serde(default = "Uuid::new_v4")]
    id: Uuid,
    food: Vec<Food>,
    recipes: Vec<Recipe>,
    portions: Vec<Portion>,
    #[serde(skip)]
    state: InventoryState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Food {
    #[serde(default = "Uuid::new_v4")]
    id: Uuid,
    name: String,
    brand: String,
    cost: usize,
    weight: usize,
    volume: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recipe {
    #[serde(default = "Uuid::new_v4")]
    id: Uuid,
    name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Portion {
    #[serde(default = "Uuid::new_v4")]
    id: Uuid,
    ingredient_id: Uuid,
    recipe_id: Uuid,
    ingredient_table: String,
    amount: usize,
}
