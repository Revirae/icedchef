use uuid::Uuid;
use serde::{Deserialize, Serialize};


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
