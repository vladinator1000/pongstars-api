use juniper::RootNode;
use lazy_static::lazy_static;

use league::{mock_league, League};
use player::{mock_player, Player};
pub use context::Context;

pub mod context;
pub mod challenge;
pub mod league;
pub mod player;

pub struct RootQuery;

#[juniper::object(Context=Context)]
impl RootQuery {
    fn current_player(_context: &Context) -> Player {
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

#[juniper::object(Context=Context)]
impl RootMutation {}

pub type Schema = RootNode<'static, RootQuery, RootMutation>;

pub fn make_schema() -> Schema {
    Schema::new(RootQuery {}, RootMutation {})
}

lazy_static! {
    pub static ref SCHEMA: Schema = make_schema();
}