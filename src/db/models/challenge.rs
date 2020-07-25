use crate::db::schema::*;
use crate::db::Connection;
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable, Debug, Identifiable)]
#[primary_key(sender, receiver)]
pub struct Challenge {
    pub sender: String,
    pub message: Option<String>,
    pub receiver: Option<String>,
    pub created_at: NaiveDateTime,
    pub expires_at: NaiveDateTime,
    pub is_ranked: bool,
}
