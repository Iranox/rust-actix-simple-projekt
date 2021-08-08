use crate::domain::user::User;

use std::vec;

use juniper::FieldResult;

pub struct UserQuery;

#[juniper::graphql_object]
impl UserQuery {
    fn findUserByName(name: String) -> FieldResult<User> {
        Ok(User {
            username: name,
            email: String::from("test@example.com"),
        })
    }

    fn findUsers() -> FieldResult<Vec<User>> {
        let users = vec![
            User {
                username: String::from("TestUser A"),
                email: String::from("test@example.com"),
            },
            User {
                username: String::from("TestUser B"),
                email: String::from("foo@example.com"),
            },
        ];
        Ok(users)
    }
}
