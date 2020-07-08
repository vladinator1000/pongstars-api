#![allow(unused)]

use chrono::NaiveDateTime;

#[derive(Queryable, Debug, Identifiable)]
#[primary_key(sender, receiver)]
pub struct Challenge {
    pub sender: String,
    pub receiver: String,
    pub created_at: NaiveDateTime,
    pub expires_at: NaiveDateTime,
}

#[derive(Queryable, Debug, Identifiable)]
pub struct Game {
    pub id: i32,
    pub tt_match: i32,
    pub score_one: i32,
    pub score_two: i32,
}

#[derive(Queryable, Debug, Identifiable)]
pub struct League {
    pub id: i32,
    pub name: String,
}

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

#[derive(Queryable, Debug, Identifiable)]
#[primary_key(player, league)]
pub struct PlayerLeague {
    pub player: String,
    pub league: i32,
}

#[derive(Queryable, Debug, Identifiable)]
pub struct Player {
    pub id: String,
    pub name: String,
}
