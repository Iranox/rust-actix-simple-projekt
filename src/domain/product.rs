use juniper::GraphQLObject;
use serde::Serialize;

use super::price::Price;

#[derive(Serialize, GraphQLObject)]
pub struct Product {
    id: String,
    description: String,
    productName: String,
    categories: Vec<String>,
    price: Price,
}

impl Product {
    pub fn new(
        id: String,
        description: String,
        productName: String,
        categories: Vec<String>,
        price: f64,
        unit: String,
    ) -> Product {
        let price = Price {
            value: price,
            unit: unit,
        };
        Product {
            id,
            description,
            productName,
            categories,
            price,
        }
    }
}
