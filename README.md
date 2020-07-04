# The back end for 🏓[Pongstars](https://github.com/jondeaves/pongstars).

If you'd like to contribute, but you're just starting out coding in Rust, check out [the book](https://doc.rust-lang.org/book/foreword.html), it's a great starting point.

## Prerequisites:
1. Rust 
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. [Docker](https://www.docker.com/get-started)


## Setup instructions

1. Set up the CLI tools
```sh
make setup
```


2. Start the database
```sh
make db
``` 


3. Run the DB migrations 
```sh
diesel migration run
```


3. Run the app and start watching for changes
```sh
make dev
```


Pongstars API uses these crates for most of the heavy lifting
- [actix_web](https://crates.io/crates/actix-web) as the HTTP server
- [juniper](https://crates.io/crates/juniper) as the GraphQL library
- [diesel](https://crates.io/crates/diesel) as the database query builder