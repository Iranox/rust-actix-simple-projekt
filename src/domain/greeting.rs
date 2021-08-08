use juniper::GraphQLObject;
use serde::Serialize;

#[derive(Serialize, GraphQLObject)]
pub struct Greeting {
    pub name: String,
}
