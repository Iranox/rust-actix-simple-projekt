#[path = "model/mod.rs"]
mod model;

use std::vec;

use juniper::FieldResult;

pub struct UserQuery;

#[juniper::graphql_object]
impl UserQuery {
    fn findUserByName(name: String) -> FieldResult<model::user::User> {
        Ok(model::user::User {
            username: name,
            email: String::from("test@example.com"),
        })
    }

    fn findUsers() -> FieldResult<Vec<model::user::User>> {
        let users = vec![
            model::user::User {
                username: String::from("TestUser A"),
                email: String::from("test@example.com"),
            },
            model::user::User {
                username: String::from("TestUser B"),
                email: String::from("foo@example.com"),
            },
        ];
        Ok(users)
    }
}
