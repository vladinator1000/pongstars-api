# The back end for 🏓[Pongstars](https://github.com/jondeaves/pongstars).

If you'd like to contribute, but you're just starting out coding in Rust, check out [the book](https://doc.rust-lang.org/book/foreword.html), it's a great starting point.

Here are [the designs](https://www.figma.com/file/wLqZVZWcI0nSSIdq4LvNXg/PongStars-app-design-iOS?node-id=0%3A1) for the mobile app.

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
make dev
```

Pongstars API uses these crates for most of the heavy lifting

- [actix_web](https://crates.io/crates/actix-web) as the HTTP server
- [juniper](https://crates.io/crates/juniper) as the GraphQL library
- [diesel](https://crates.io/crates/diesel) as the database query builder

### References

- https://www.experttabletennis.com/table-tennis-rules-and-regulations/
