pub mod challenge;
pub mod game;
pub mod league;
pub mod player;
pub mod tt_match;

#[derive(Debug)]
pub struct PlayerLeague {
    pub player: String,
    pub league: i32,
}
