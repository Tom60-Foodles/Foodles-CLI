use serde::Deserialize;

// Structures pour désérialiser le JSON

#[derive(Debug, Deserialize)]
pub struct Fridge {
    pub categories: Vec<Category>,
}

#[derive(Debug, Deserialize)]
pub struct Category {
    pub name: String,
    pub slug: String,
    pub products: Vec<Product>,
}

#[derive(Debug, Deserialize)]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub image: String,
    pub image_id: String,
    pub tags: Vec<Tag>,
    pub icons: Vec<Icon>,
    pub ingredients: String,
    pub allergen_list: Vec<String>,
    pub trace_allergens: Vec<String>,
    pub allergens: String,
    pub weight: u32,
    pub volume: u32,
    pub quantity: u32,
    pub price: Price,
    pub has_near_expiration_sale: bool,
    pub energy_kj: u32,
    pub energy_kcal: u32,
    pub fat_content: f32,
    pub fat_content_ags: f32,
    pub glucides: f32,
    pub glucides_sugar: f32,
    pub fibres: f32,
    pub proteins: f32,
    pub salt: f32,
    pub nutriscore: String,
}

#[derive(Debug, Deserialize)]
pub struct Tag {
    pub name: String,
    pub slug: String,
}

#[derive(Debug, Deserialize)]
pub struct Icon {
    pub id: u32,
    pub name: String,
    pub slug: String,
}

#[derive(Debug, Deserialize)]
pub struct Price {
    pub amount: u32,
    pub currency: String,
}



