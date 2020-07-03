use actix_web::{get, post, web, Error, HttpResponse};
use juniper::{http::graphiql::graphiql_source, http::GraphQLRequest};
use std::sync::Arc;

use super::{context::GraphQLContext, Schema};
use crate::db::pool::PostgresPool;

#[get("/")]
pub async fn playground() -> HttpResponse {
    let html = graphiql_source("/graphql");

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[post("/graphql")]
pub async fn graphql(
    db: web::Data<PostgresPool>,
    schema: web::Data<Arc<Schema>>,
    query: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let context = GraphQLContext {
        db: db.get_ref().to_owned(),
    };

    let result = web::block(move || {
        let gql_result = query.execute(&schema, &context);
        let json = serde_json::to_string(&gql_result);
        Ok::<_, serde_json::error::Error>(json?)
    })
    .await
    .map_err(Error::from)?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(result))
}

pub fn configure(config: &mut web::ServiceConfig) {
    let schema = Arc::new(super::make_schema());

    config.data(schema).service(playground).service(graphql);
}
