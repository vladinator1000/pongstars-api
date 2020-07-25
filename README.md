# The back end for 🏓 [Pongstars](https://github.com/jondeaves/pongstars).
Pongstars is a table tennis mobile app made in [Flutter](https://flutter.dev/). Here's how it [looks like](https://www.figma.com/file/wLqZVZWcI0nSSIdq4LvNXg/PongStars-app-design-iOS?node-id=0%3A1).


If you'd like to contribute, but you're just starting out coding in Rust, check out [the book](https://doc.rust-lang.org/book/foreword.html), it's a great starting point.


## Prerequisites:

1. [Docker](https://www.docker.com/get-started)

1. (Mac only)

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
cargo run
```

Pongstars API uses these crates for most of the heavy lifting

- [actix_web](https://crates.io/crates/actix-web) as the HTTP server
- [juniper](https://crates.io/crates/juniper) as the GraphQL library
- [diesel](https://crates.io/crates/diesel) as the database query builder

To generate data models from the database for reference 

```sh
diesel_ext > src/db/reference_models.rs
```

⚠ This is not used within the actual code, because it's split in separate modules. It's useful to know this command if you change the database schema and would like to auto-generate models that you can copy in your code.

### References

- https://www.experttabletennis.com/table-tennis-rules-and-regulations/
