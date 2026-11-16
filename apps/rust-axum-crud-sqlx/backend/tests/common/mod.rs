use postgres_testcontainer::postgres::setup_postgres_container;
use sqlx::PgPool;

/// Returns a pool connected to an isolated schema in the shared Postgres
/// testcontainer, with all migrations applied.
pub async fn setup_pool() -> anyhow::Result<PgPool> {
    let guard = setup_postgres_container().await?;
    sqlx::migrate!("./migrations").run(&guard.pg_pool).await?;
    Ok(guard.pg_pool)
}

/// Basic auth header value for `test:test`.
pub const BASIC_AUTH: &str = "Basic dGVzdDp0ZXN0";
