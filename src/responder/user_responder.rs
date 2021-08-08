use crate::domain::user::User;

use std::vec;

use juniper::FieldResult;

pub struct UserQuery;

#[juniper::graphql_object]
impl UserQuery {
    fn findUserByName(name: String) -> FieldResult<User> {
        Ok(User::new(name, String::from("test@email.com")))
    }

    fn findUsers() -> FieldResult<Vec<User>> {
        let users = vec![
            User::new(String::from("TestUser A"), String::from("test@email.com")),
            User::new(String::from("TestUser B"), String::from("foo@email.com")),
        ];
        Ok(users)
    }
}
