use diesel::prelude::*;
use crate::db::Connection;
use crate::db::schema::*;

#[derive(Queryable, Debug, Identifiable)]
pub struct Game {
    pub id: i32,
    pub tt_match: i32,
    pub score_one: i32,
    pub score_two: i32,
}