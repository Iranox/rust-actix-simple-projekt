use juniper::FieldResult;

use crate::domain::greeting::Greeting;

pub struct GreetingQuery;

#[juniper::graphql_object]
impl GreetingQuery {
    fn hello(&self) -> FieldResult<Greeting> {
        Ok(Greeting {
            name: "Welt".to_string(),
        })
    }
}
