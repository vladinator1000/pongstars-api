use crate::db::schema::*;
use crate::db::Connection;
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable, Debug, Identifiable)]
#[primary_key(sender, receiver)]
pub struct Challenge {
    pub sender: String,
    pub receiver: String,
    pub created_at: NaiveDateTime,
    pub expires_at: NaiveDateTime,
}
