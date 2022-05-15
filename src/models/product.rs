use diesel::{Queryable, Insertable};
use serde::Serialize;
use crate::schema::products;

#[derive(Queryable, Serialize, Clone)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub image: String,
    pub description: String,
    pub price: i32,
}

#[derive(Insertable)]
#[diesel(table_name = products)]
pub struct NewProduct<'a> {
    pub name: &'a str,
    pub image: &'a str,
    pub description: &'a str,
    pub price: i32,
}
