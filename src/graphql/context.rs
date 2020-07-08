use crate::db::pool::PostgresPool;

pub struct GraphQLContext {
    pub db: PostgresPool,
}

impl juniper::Context for GraphQLContext {}
