use super::pool;

#[async_std::test]
async fn pool_connects_to_db() -> sqlx::Result<()> {
    let pool = pool::get_pool().await.unwrap();

    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool)
        .await?;

    assert_eq!(row.0, 150);

    Ok(())
}
