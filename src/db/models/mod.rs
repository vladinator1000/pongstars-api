use super::schema::*;

pub mod challenge;
pub mod game;
pub mod league;
pub mod player;
pub mod tt_match;

#[derive(Queryable, Debug, Identifiable)]
#[primary_key(player, league)]
#[table_name = "player_league"]
pub struct PlayerLeague {
    pub player: String,
    pub league: i32,
}
