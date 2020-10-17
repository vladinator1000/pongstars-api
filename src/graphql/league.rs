use super::player::*;
pub use crate::db::models::league::League;

#[juniper::graphql_object]
impl League {
    fn id(&self) -> i32 {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn players(&self) -> Vec<Player> {
        vec![mock_player()]
    }
}

pub fn mock_league() -> League {
    League {
        id: 1,
        name: "GLA".into(),
    }
}
