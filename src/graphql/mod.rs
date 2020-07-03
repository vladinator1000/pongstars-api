pub mod context;
pub mod endpoints;
pub mod user;

use juniper::RootNode;

use context::GraphQLContext;
use user::User;

pub struct RootQuery;

#[juniper::object(Context=GraphQLContext)]
impl RootQuery {
    fn current_user(_context: &GraphQLContext) -> User {
        User {
            id: 1,
            name: "Jon".to_string(),
        }
    }
}

pub struct RootMutation;

#[juniper::object(Context=GraphQLContext)]
impl RootMutation {}

pub type Schema = RootNode<'static, RootQuery, RootMutation>;

pub fn make_schema() -> Schema {
    Schema::new(RootQuery {}, RootMutation {})
}
