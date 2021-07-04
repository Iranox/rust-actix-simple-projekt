use juniper::FieldResult;
use juniper::{EmptySubscription, RootNode};
use std::fs::File;
use std::io::prelude::*;

#[derive(GraphQLEnum)]
enum Episode {
    NewHope,
    Empire,
    Jedi,
}


use juniper::{GraphQLEnum, GraphQLInputObject, GraphQLObject};

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct Human {
    id: String,
    name: String,
    appears_in: Vec<Episode>,
    home_planet: String,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct NewHuman {
    name: String,
    appears_in: Vec<Episode>,
    home_planet: String,
}

pub struct QueryRoot;

#[juniper::graphql_object]
impl QueryRoot {
    fn human(_id: String) -> FieldResult<Human> {
        Ok(Human {
            id: "1234".to_owned(),
            name: "Luke".to_owned(),
            appears_in: vec![Episode::NewHope],
            home_planet: "Mars".to_owned(),
        })
    }
}

pub struct MutationRoot;

#[juniper::graphql_object]
impl MutationRoot {
    fn create_human(new_human: NewHuman) -> FieldResult<Human> {
        Ok(Human {
            id: "1234".to_owned(),
            name: new_human.name,
            appears_in: new_human.appears_in,
            home_planet: new_human.home_planet,
        })
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription>;

fn write_schema_to_file(ddl_schema: &String)->std::io::Result<()> {
    let mut file = File::create("schema.graphql")?;
    file.write_all(ddl_schema.as_bytes())?;
    Ok(())
}

pub fn create_schema() -> Schema {
    let schema = Schema::new(QueryRoot {}, MutationRoot {}, EmptySubscription::new());
    write_schema_to_file(&schema.as_schema_language());
    Schema::new(QueryRoot {}, MutationRoot {}, EmptySubscription::new())
}
