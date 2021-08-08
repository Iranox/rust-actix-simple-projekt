use juniper::GraphQLObject;
use serde::Serialize;


#[derive(Serialize, GraphQLObject)]
pub struct Price{
    pub value: f64,
    pub unit: String,
}


impl Price {
    pub fn to_string(&self) -> String {
        format!("{} {}", &self.value, &self.unit)
    }
}
