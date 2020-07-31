use crate::db::schema::*;
use crate::db::{Connection};
use diesel::prelude::*;

#[derive(Queryable, Debug, Identifiable)]
#[primary_key(player, league)]
#[table_name = "player_league"]
pub struct PlayerLeague {
  pub player: String,
  pub league: i32,
  pub matchmaking_rating: i32,
  pub matches_played: i32,
  pub wins: i32,
  pub losses: i32,
}
