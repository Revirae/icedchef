use crate::models::{Food, Portion, Recipe};


#[derive(Default, Debug)]
pub struct AppState {
    pub food: Vec<Food>,
    pub recipes: Vec<Recipe>,
    pub portions: Vec<Portion>,
    pub dirty: bool,
    pub saving: bool
}

#[derive(Default, Debug, Clone)]
pub enum InventoryState {
    #[default] Idle,
    EditingFood,
}
