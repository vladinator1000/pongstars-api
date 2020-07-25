use super::{
    challenge::{Challenge},
    league::{League},
    mocks::{mock_challenge, mock_league, mock_date_time, mock_stats},
};
pub use crate::{DateTimeUtc, db::models::player::Player};

#[juniper::object]
impl Player {
    fn id(&self) -> &str {
        &self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn leagues(&self) -> Vec<League> {
        vec![mock_league()]
    }

    fn challenges() -> Vec<Challenge> {
        vec![mock_challenge()]
    }

    fn joined() -> DateTimeUtc {
        mock_date_time()
    }

    fn stats(&self, league_id: i32) -> Stats {
        mock_stats()
    }
}

#[derive(juniper::GraphQLObject)]
pub struct Stats {
    pub matchmaking_rating: i32,
    pub matches_played: i32,
    pub wins: i32,
    pub losses: i32,
}
