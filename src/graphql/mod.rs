pub mod challenge;
pub mod context;
pub mod endpoints;
pub mod league;
pub mod player;
pub mod table_tennis_match;
pub mod auth;
pub mod mocks;

use juniper::RootNode;
use context::GraphQLContext;
use league::{League};
use player::{Player};
use mocks::{mock_league, mock_date_time, mock_player};
use crate::DateTimeUtc;

pub struct RootQuery;

#[juniper::object(Context=GraphQLContext)]
impl RootQuery {
    fn current_player(_context: &GraphQLContext) -> Player {
        mock_player()
    }

    fn leagues(_context: &GraphQLContext) -> Vec<League> {
        vec![mock_league()]
    }
}

pub struct RootMutation;

#[juniper::object(Context=GraphQLContext)]
impl RootMutation {}

pub type Schema = RootNode<'static, RootQuery, RootMutation>;

pub fn make_schema() -> Schema {
    Schema::new(RootQuery {}, RootMutation {})
}
