use diesel::prelude::*;
use crate::db::Connection;
use crate::db::schema::*;

#[derive(Queryable, Debug, Identifiable)]
pub struct League {
    pub id: i32,
    pub name: String,
}
