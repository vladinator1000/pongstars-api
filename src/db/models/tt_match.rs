use diesel::prelude::*;
use chrono::NaiveDateTime;
use crate::db::Connection;
use crate::db::schema::*;

#[derive(Queryable, Debug, Identifiable)]
#[table_name = "matches"]
pub struct Match {
    pub id: i32,
    pub league: i32,
    pub player_one: String,
    pub player_two: String,
    pub length: i32,
    pub created_at: NaiveDateTime,
    pub played_at: Option<NaiveDateTime>,
}