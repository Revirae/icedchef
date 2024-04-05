use crate::models::{FoodForm, Inventory};


#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum Tab {
    #[default]
    Food,
    Recipe
}

#[derive(Default, Debug)]
pub struct AppState {
    pub inventory: Inventory,
    pub food_form: FoodForm,
    pub current_tab: Tab,
    pub dirty: bool,
    pub saving: bool
}

#[derive(Default, Debug, Clone)]
pub struct InventoryState {
}
