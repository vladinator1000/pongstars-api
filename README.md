This is the back end for the 🏓[Pongstars app](https://github.com/jondeaves/pongstars).

If you'd like to contribute, but you're just starting out coding in Rust, check out [the book](https://doc.rust-lang.org/book/foreword.html), it's a great starting point.

To get started developing, you'll need the following prerequisites:

1. Install Rust 
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Install [Docker](https://www.docker.com/get-started)


## To set up the app
1. Set up the database and required libraries
```
make db
``` 

2. Run the app and start watching for changes
```
make dev
```

Pongstars API uses these crates for most of the heavy lifting
- [actix_web](https://crates.io/crates/actix-web) as the HTTP server
- [juniper](https://crates.io/crates/juniper) as the GraphQL library
- [diesel](https://crates.io/crates/diesel) as the database query builder