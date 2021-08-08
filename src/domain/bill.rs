use chrono::{DateTime, NaiveDateTime, Utc};
use juniper::GraphQLObject;
use serde::Serialize;

use super::{addresses::Addresses, price::Price, product::Product, user::User};

#[derive(Serialize, GraphQLObject)]
pub struct Bill {
    user: User,
    products: Vec<Product>,
    billing_address: Addresses,
    final_price: Price,
    date: DateTime<Utc>,
}

impl Bill {
    pub fn new(username: String) -> Bill {
        let addresses = Addresses {
            street: "Test".to_string(),
            housenummer: "12b".to_string(),
            zip: "00000".to_string(),
            country: "Germany".to_string(),
            state: None,
        };
        let products = vec![
            Product::new(
                String::from("123123"),
                String::from("aslfdjjl"),
                String::from(""),
                vec![String::from("Placeholder")],
                12.56,
                String::from("€"),
            ),
            Product::new(
                String::from("123123"),
                String::from("aslfdjjl"),
                String::from(""),
                vec![String::from("Placeholder")],
                123.45,
                String::from("€"),
            ),
        ];
        Bill {
            user: User {
                username,
                email: String::from("test@gmail.com"),
                addresses,
            },
            products,
            final_price: Price {
                value: 1.0,
                unit: String::from("€"),
            },
            billing_address: Addresses {
                street: "Test".to_string(),
                housenummer: "12b".to_string(),
                zip: "00000".to_string(),
                country: "Germany".to_string(),
                state: None,
            },
            date: chrono::offset::Utc::now(),
        }
    }
}
