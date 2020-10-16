
use tide::{http::mime, Body, Redirect, Request, Response, Server, StatusCode};
use juniper::{http::{GraphQLRequest, graphiql}};

mod db;
mod graphql;

pub type DateTime = chrono::DateTime<chrono::Utc>;

const HOST: &str = "0.0.0.0:3029";

async fn handle_graphql(mut request: Request<graphql::Context>) -> tide::Result {
    let query: GraphQLRequest = request.body_json().await?;
    let response = query.execute(&graphql::SCHEMA, request.state());
    let status = if response.is_ok() {
        StatusCode::Ok
    } else {
        StatusCode::BadRequest
    };

    Ok(Response::builder(status)
        .body(Body::from_json(&response)?)
        .build())
}

async fn handle_graphiql(_: Request<graphql::Context>) -> tide::Result<impl Into<Response>> {
    Ok(Response::builder(200)
        .body(graphiql::graphiql_source("/graphql"))
        .content_type(mime::HTML))
}

#[async_std::main]
async fn main() -> std::io::Result<()> {
    let mut app = Server::with_state(graphql::Context {});

    
    app.at("/").get(Redirect::permanent("/graphiql"));
    app.at("/graphql").post(handle_graphql);
    app.at("/graphiql").get(handle_graphiql);
    
    app.listen(HOST).await?;

    Ok(())
}