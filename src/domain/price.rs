use juniper::GraphQLObject;
use serde::Serialize;


#[derive(Serialize, GraphQLObject)]
pub struct Price{
    pub value: f64,
    pub unit: String,
}
