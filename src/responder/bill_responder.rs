use juniper::FieldResult;

use crate::domain::bill::Bill;


pub struct BillQuery;

#[juniper::graphql_object]
impl BillQuery {
    fn bill_by_user_name(user_name: String)->FieldResult<Vec<Bill>>{
        Ok(vec![Bill::new(user_name)])
    }
}
