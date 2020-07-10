pub mod context;
pub mod endpoints;
pub mod player;

use juniper::RootNode;
use context::GraphQLContext;

pub struct RootQuery;

pub struct RootMutation;

#[juniper::object(Context=GraphQLContext)]
impl RootMutation {}

pub type Schema = RootNode<'static, RootQuery, RootMutation>;

pub fn make_schema() -> Schema {
    Schema::new(RootQuery {}, RootMutation {})
}
