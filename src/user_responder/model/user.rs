use juniper::GraphQLObject;
use serde::Serialize;

#[derive(Serialize, GraphQLObject)]
pub struct User {
    pub username: String,
    pub email: String,
}
