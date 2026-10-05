use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};

// A connection pool, not a single connection: this lets multiple concurrent
// requests borrow a connection each, hand it back when done, and reuse it —
// same idea as Arc<Mutex<T>> conceptually (shared, coordinated access to a
// limited resource), but purpose-built for database connections.
pub async fn init_pool(database_url: &str) -> Result<SqlitePool, sqlx::Error> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    // Create the table if it doesn't exist yet. In a real project you'd use
    // `sqlx migrate` with versioned migration files instead of inline SQL
    // like this — this is simplified for a self-contained reference.
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS tasks (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            description TEXT NOT NULL,
            done        BOOLEAN NOT NULL DEFAULT 0
        )
        "#,
    )
    .execute(&pool)
    .await?;

    Ok(pool)
}
