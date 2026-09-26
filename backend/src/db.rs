use sqlx::{postgres::PgPool, sqlite::SqlitePool, Pool, Pool_Type};
use std::env;

pub type DbPool = sqlx::Pool<sqlx::Pool_Type>; // This is a placeholder for the actual pool type.
// Actually, since we have both features, we might need a common type or decide on one.
// For now, I'll use a common type or a specific one if we decide on the DB.

// To keep it simple for now, I'll assume we might use PostgreSQL or SQLite.
// But sqlx uses different types for each. Let's use a common approach for the structure.

pub type DbPool = sqlx::Pool<sqlx::postgres::PgPool>; // Defaulting to Postgres for now as it's standard for production.

pub async fn init_db_pool(db_url: &str) -> Result<DbPool, sqlx::Error> {
    let pool = sqlx::PgPool::connect(db_url).await?;
    Ok(pool)
}
