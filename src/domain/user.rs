use juniper::GraphQLObject;
use serde::Serialize;

use super::addresses::Addresses;


#[derive(Serialize, GraphQLObject)]
pub struct User {
    pub username: String,
    pub email: String,
    pub addresses: Addresses,
}

impl User {
    pub fn new(user_name: String, email: String)->User{
        let addresses = Addresses{
            street:"Test".to_string(),
            housenummer: "12b".to_string(),
            zip: "00000".to_string(),
            country:"Germany".to_string(),
            state: None,
        };
        User {
            username: user_name,
            email,
            addresses,
        }
    }
}
