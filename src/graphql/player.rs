use super::league::{League, mock_league};
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
}

pub fn mock_player() -> Player {
    Player {
        id: "1".into(),
        name: "Jon".into(),
    }
}
