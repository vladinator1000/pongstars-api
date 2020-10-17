use juniper::{EmptySubscription, RootNode};
use lazy_static::lazy_static;

pub use context::Context;
use league::{mock_league, League};
use player::{mock_player, Player};

pub mod challenge;
pub mod context;
pub mod league;
pub mod player;

pub struct RootQuery;

#[juniper::graphql_object(Context=Context)]
impl RootQuery {
    async fn current_player(context: &Context) -> Player {
        mock_player()
    }

    fn log_in(_context: &Context) -> Player {
        mock_player()
    }

    fn leagues(_context: &Context) -> Vec<League> {
        vec![mock_league()]
    }
}

pub struct RootMutation;

#[juniper::graphql_object(Context=Context)]
impl RootMutation {
    fn hello() -> &str {
        "world"
    }
}

pub type Schema = RootNode<'static, RootQuery, RootMutation, EmptySubscription<Context>>;

pub fn make_schema() -> Schema {
    Schema::new(RootQuery {}, RootMutation {}, EmptySubscription::new())
}

lazy_static! {
    pub static ref SCHEMA: Schema = make_schema();
}
