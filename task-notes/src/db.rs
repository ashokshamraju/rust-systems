use sqlx::{sqlite::{SqlitePool, SqlitePoolOptions}};

pub async fn init_pool(database_url: &str)->Result<SqlitePool, sqlx::Error> {
    let pool = SqlitePoolOptions::new()
                .max_connections(5)
                .connect(database_url)
                .await?;
    
    sqlx::query(r#" 
        CREATE TABLE IF NOT EXISTS tasks (
        id  INTEGER PRIMARY KEY AUTOINCREMENT,
        title TEXT NOT NULL,
        body TEXT NOT NULL,
        pinned BOOLEAN NOT NULL DEFAULT 0
        )
        "#,
    )
    .execute(&pool)
    .await?;

    Ok(pool)
}