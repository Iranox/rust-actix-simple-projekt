use juniper::EmptyMutation;
use juniper::{EmptySubscription, RootNode};
use std::fs::File;
use std::io::prelude::*;

use crate::responder::bill_responder::BillQuery;
use crate::responder::greeting_responder::GreetingQuery;
use crate::responder::product_responder::ProductQuery;
use crate::responder::user_responder::UserQuery;

#[derive(juniper::GraphQLObject)]
pub struct QueryRoot {
    greeting: GreetingQuery,
    user: UserQuery,
    product: ProductQuery,
    bill: BillQuery
}

impl QueryRoot {
    pub fn new() -> Self {
        Self {
            greeting: GreetingQuery,
            user: UserQuery,
            product: ProductQuery,
            bill: BillQuery
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
