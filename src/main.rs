#[macro_use]
extern crate diesel;

use actix_web::{App, HttpServer};

mod db;
mod graphql;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let db = db::pool::get_pool();
    let address = "127.0.0.1:3232";
    println!("Starting server at http://{}", address);

    let server = HttpServer::new(move || {
        App::new()
            .data(db.clone())
            .configure(graphql::endpoints::configure)
    });

    server.bind(address)?.run().await
}
