use crate::db::pool::PostgresPool;

pub struct GraphQLContext {
    pub db: PostgresPool,
}

// This impl allows us to pass in GraphQLContext as the Context for GraphQL
// objects
impl juniper::Context for GraphQLContext {}
