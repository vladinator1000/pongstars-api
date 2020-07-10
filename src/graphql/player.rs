use super::{
    challenge::{mock_challenge, Challenge},
    league::{mock_league, League},
};
pub use crate::db::models::player::*;

#[juniper::object]
impl Player {
    fn id(&self) -> &str {
        &self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn token(&self) -> &str {
        "token"
    }

    fn leagues(&self) -> Vec<League> {
        vec![mock_league()]
    }

    fn challenges() -> Vec<Challenge> {
        vec![mock_challenge()]
    }
}

pub fn mock_player() -> Player {
    Player {
        id: "1".into(),
        name: "Jon".into(),
    }
}
