# The back end for 🏓 [Pongstars](https://github.com/jondeaves/pongstars).
Pongstars is a table tennis mobile app made in [Flutter](https://flutter.dev/). Here's how it [looks like](https://www.figma.com/file/wLqZVZWcI0nSSIdq4LvNXg/PongStars-app-design-iOS?node-id=0%3A1).


If you'd like to contribute, but you're just starting out coding in Rust, check out [the book](https://doc.rust-lang.org/book/foreword.html), it's a great starting point.

## Pre-requisites

1. [Docker](https://www.docker.com/get-started)

1. OpenSSL (Mac only)

```
brew install openssl
```

## Setup instructions

1. Set up the CLI tools

```sh
make setup
```

1. Run the app, it will start watching for changes

```sh
make dev
```

## Testing

Run the unit tests
```sh
cargo test
```

Run the integration tests
```sh
cargo test --features "integration_tests"
```

## Additional info

Run in release mode
```sh
cargo run --release
```


Build in release mode
```sh
cargo build --release
```

Pongstars API uses these crates for most of the heavy lifting

- [Tide](https://github.com/http-rs/tide) as the HTTP server
- [Juniper](https://crates.io/crates/juniper) as the GraphQL library
- [SQLx](https://github.com/launchbadge/sqlx) for talking to the database

### References

- https://graphql.org/learn/
- https://www.experttabletennis.com/table-tennis-rules-and-regulations/