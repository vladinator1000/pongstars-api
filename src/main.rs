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

    let mut listenfd = listenfd::ListenFd::from_env();

    let mut server = HttpServer::new(move || {
        App::new()
            .data(db.clone())
            .configure(graphql::endpoints::configure)
    });

    // Dev mode autoreloading or fall back to normal bind
    // https://actix.rs/docs/autoreload/
    server = if let Some(listener) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(listener)?
    } else {
        server.bind(address)?
    };

    server.run().await
}
