use crate::db::schema::*;
use crate::db::Connection;
use diesel::prelude::*;

#[derive(Queryable, Debug, Identifiable)]
pub struct League {
    pub id: i32,
    pub name: String,
}
