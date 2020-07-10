use super::player::*;
use crate::db::models::League::*;
use super::{RootQuery, GraphQLContext};

#[juniper::object]
impl League {
    fn id(&self) -> i32 {
        &self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn players(&self) -> Vec<Player> {
      vec![mock_player()]
    }
}


fn mock_league() -> League {
    League {
        id: "1".into(),
        name: "GLA".into(),
    }
}

#[juniper::object(Context=GraphQLContext)]
impl RootQuery {
    fn leagues(_context: &GraphQLContext) -> Vec<League> {
         vec![mock_league()]
    }
}