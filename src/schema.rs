use juniper::EmptyMutation;
use juniper::{EmptySubscription, RootNode};
use std::fs::File;
use std::io::prelude::*;

use crate::hello_responder::hello::GreetingQuery;
use crate::user_responder::user_handler::UserQuery;

#[derive(juniper::GraphQLObject)]
pub struct QueryRoot {
    greeting: GreetingQuery,
    user: UserQuery,
}

impl QueryRoot {
    pub fn new() -> Self {
        Self {
            greeting: GreetingQuery,
            user: UserQuery,
        }
    }
}

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation, EmptySubscription>;

fn write_schema_to_file(ddl_schema: &String) -> std::io::Result<()> {
    let mut file = File::create("schema.graphql")?;
    file.write_all(ddl_schema.as_bytes())?;
    Ok(())
}

pub fn create_schema() -> Schema {
    let schema = Schema::new(
        QueryRoot::new(),
        EmptyMutation::new(),
        EmptySubscription::new(),
    );
    write_schema_to_file(&schema.as_schema_language());
    schema
}
