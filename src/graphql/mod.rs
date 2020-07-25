pub mod challenge;
pub mod context;
pub mod endpoints;
pub mod league;
pub mod player;
pub mod auth;

use context::GraphQLContext;
use juniper::RootNode;
use league::{mock_league, League};
use player::{mock_player, Player};
use auth::{mock_login_result, LoginResult, LoginInput, SignupInput};

pub struct RootQuery;

#[juniper::object(Context=GraphQLContext)]
impl RootQuery {
    fn current_player(_context: &GraphQLContext) -> Player {
        mock_player()
    }

    fn log_in(_context: &GraphQLContext, input: LoginInput) -> LoginResult {
        mock_login_result()
        
    }
    
    fn sign_up(_context: &GraphQLContext, input: SignupInput) -> bool {
        true
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
