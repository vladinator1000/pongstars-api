use super::{context::GraphQLContext, mocks::mock_player, player::Player};
pub use crate::db::models::league::League;

#[juniper::object(Context=GraphQLContext)]
impl League {
    fn id(&self) -> i32 {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn members(&self) -> Vec<Player> {
        vec![mock_player()]
    }
}
