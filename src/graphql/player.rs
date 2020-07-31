use super::{
    context::GraphQLContext,
    challenge::{Challenge},
    league::{League},
    mocks::{mock_challenge, mock_league, mock_date_time, mock_stats, mock_table_tennis_match},
};
pub use crate::{DateTimeUtc, db::{table_tennis_match::TableTennisMatch, models::player::Player}};

#[juniper::object(Context=GraphQLContext)]
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

    fn matches(&self) -> Vec<TableTennisMatch> {
        vec![mock_table_tennis_match()]
    }
}

#[derive(juniper::GraphQLObject)]
pub struct Stats {
    pub matchmaking_rating: i32,
    pub matches_played: i32,
    pub wins: i32,
    pub losses: i32,
}
