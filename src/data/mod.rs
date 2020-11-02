use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Cocktail {
    pub name: String,
    pub constituents: Vec<Constituent>,
    pub instructions: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Constituent {
    pub ingredient: Ingredient,
    pub quantity: Quantity,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Ingredient {
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Quantity {
    pub unit: Unit,
    pub amount: f32,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Unit {
    BarSpoon,
    Millilitres,
    Grams,
    Ounces,
    Unit,
}
