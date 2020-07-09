use chrono::NaiveDateTime;
use diesel::prelude::*;
use crate::db::Connection;
use crate::db::schema::*;

#[derive(Queryable, Debug, Identifiable)]
#[primary_key(sender, receiver)]
pub struct Challenge {
    pub sender: String,
    pub receiver: String,
    pub created_at: NaiveDateTime,
    pub expires_at: NaiveDateTime,
}