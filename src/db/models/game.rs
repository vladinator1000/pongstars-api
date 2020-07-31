use crate::db::schema::*;
use crate::db::Connection;
use diesel::prelude::*;

#[derive(Queryable, Debug, Identifiable)]
pub struct Game {
    pub id: i32,
    pub table_tennis_match: i32,
    pub score_one: i32,
    pub score_two: i32,
}
