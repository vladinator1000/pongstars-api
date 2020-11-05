For unit testing, please refer to the [testing chapter in the Rust book](https://doc.rust-lang.org/book/ch11-00-testing.html). 

To add an integration test from a file, you can annotate the test module with the cfg attribute macro. For example, here's an asynchronous test that talks to the database.

```rs
#[cfg(test)]
#[cfg(feature = "integration_tests")]
mod db_integration_tests;
```

and put the tests in that file

```rs
#[async_std::test]
async fn pool_connects_to_db() -> sqlx::Result<()> {
    let pool = pool::get_pool().await.unwrap();

    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool)
        .await?;

    assert_eq!(row.0, 150);
}
```

This puts the module behind a feature flag, and tells cargo to not use it if the flag isn't present.

You can do the same thing in a file.

```rs
#[cfg(test)]
#[cfg(feature = "integration_tests")]
mod db_integration_tests {
  #[async_std::test]
  async fn pool_connects_to_db() -> sqlx::Result<()> {
      let pool = pool::get_pool().await.unwrap();

      let row: (i64,) = sqlx::query_as("SELECT $1")
          .bind(150_i64)
          .fetch_one(&pool)
          .await?;

      assert_eq!(row.0, 150);
  }
}
```
