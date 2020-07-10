use crate::db::models::player::*;
use super::{RootQuery, GraphQLContext};


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
}


pub fn mock_player() -> Player {
    Player {
        id: "1".into(),
        name: "Jon".into(),
    }
}

#[juniper::object(Context=GraphQLContext)]
impl RootQuery {
    fn current_player(_context: &GraphQLContext) -> Player {
         mock_player()
    }

    fn log_in(_context: &GraphQLContext) -> Player {
        mock_player()
    }
}