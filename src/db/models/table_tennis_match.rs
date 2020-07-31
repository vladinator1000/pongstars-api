use crate::db::schema::*;
use crate::db::Connection;
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable, Debug, Identifiable)]
#[table_name = "matches"]
pub struct TableTennisMatch {
    pub id: i32,
    pub league: i32,
    pub player_one: String,
    pub player_two: String,
    pub length: i32,
    pub created_at: NaiveDateTime,
    pub played_at: Option<NaiveDateTime>,
}
