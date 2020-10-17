use pongstars_api::db::get_pool;

// An example of an asynchronous test
#[async_std::test]
async fn pool_connects_to_db() -> sqlx::Result<()> {
    let pool = get_pool().await.unwrap();

    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool)
        .await?;

    assert_eq!(row.0, 150);

    Ok(())
}
