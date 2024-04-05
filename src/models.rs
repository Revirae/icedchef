use iced::{widget::{text_input, Row}, Element};
use iced_aw::{number_input, NumberInput, NumberInputStyles};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use crate::{messages::FormMessage, states::InventoryState};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Inventory {
    #[serde(default = "Uuid::new_v4")]
    id: Uuid,
    food: Vec<Food>,
    recipes: Vec<Recipe>,
    portions: Vec<Portion>,
    #[allow(dead_code)]
    #[serde(skip)]
    state: InventoryState,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Food {
    #[serde(default = "Uuid::new_v4")]
    id: Uuid,
    name: String,
    brand: String,
    cost: usize,
    weight: usize,
    volume: usize,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Recipe {
    #[serde(default = "Uuid::new_v4")]
    id: Uuid,
    name: String,
    #[serde(skip)]
    portions: Vec<Portion>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Portion {
    #[serde(default = "Uuid::new_v4")]
    id: Uuid,
    ingredient_id: Uuid,
    ingredient_table: String,
    recipe: Recipe,
    amount: usize,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct FoodForm {
    name: String,
    brand: String,
    cost: f64,
    weight: f64,
    volume: f64,
}

impl FoodForm {
    pub fn update(&mut self, message: FormMessage) {
        match message {
            FormMessage::NameChanged(name) => {
                self.name = name;
            }
            FormMessage::BrandChanged(brand) => {
                self.brand = brand;
            }
            FormMessage::CostChanged(cost) => {
                self.cost = cost;
            }
            FormMessage::WeightChanged(weight) => {
                self.weight = weight;
            }
            FormMessage::VolumeChanged(volume) => {
                self.volume = volume;
            }
            // _ => {}
        }
    }
    pub fn view(&self) -> Element<FormMessage> {
        let form_name_input = text_input("Nome", self.name.as_ref());
        let form_brand_input = text_input("Marca", self.brand.as_ref());
        let form_cost_input: NumberInput<'_, f64, FormMessage> = number_input(
            self.cost,
            9999.,
            FormMessage::CostChanged
        ).style(NumberInputStyles::Default)
         .step(0.05);
        let form_weight_input: NumberInput<'_, f64, FormMessage> = number_input(
            self.weight,
            9999.,
            FormMessage::WeightChanged
        ).style(NumberInputStyles::Default)
         .step(0.1);
        let form_volume_input: NumberInput<'_, f64, FormMessage> = number_input(
            self.volume,
            9999.,
            FormMessage::VolumeChanged
        ).style(NumberInputStyles::Default)
         .step(0.1);
    
        Row::new()
            .push(form_name_input)
            .push(form_brand_input)
            .push(form_cost_input)
            .push(form_weight_input)
            .push(form_volume_input)
            .into()

        // Column::new()
            // .push(form)
            // .into()
    }
}

#[allow(dead_code)]
trait Ingredient {}

impl Ingredient for Food {}
impl Ingredient for Recipe {}
