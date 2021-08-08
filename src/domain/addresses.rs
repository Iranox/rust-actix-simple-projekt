use juniper::GraphQLObject;
use serde::Serialize;

#[derive(Serialize, GraphQLObject)]
pub struct Addresses {
    pub zip: String,
    pub street: String,
    pub housenummer:String,
    pub country:String,
    pub state: Option<String>,
}
