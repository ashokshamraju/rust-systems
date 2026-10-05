mod db;
mod error;
mod handlers;
mod models;

use axum::{
    routing::{delete, get, post, put},
    Router,
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // `sqlite::memory:` = in-memory DB, wiped on restart — convenient for
    // trying this out. Swap to `sqlite:tasks.db` for a file that persists
    // between runs.
    let pool = db::init_pool("sqlite::memory:")
        .await
        .expect("failed to set up database");

    // Each route maps an HTTP method + path to a handler function. Note how
    // this reads almost like a table of contents for the whole API.
    let app = Router::new()
        .route("/tasks", get(handlers::list_tasks))
        .route("/tasks", post(handlers::create_task))
        .route("/tasks/:id", get(handlers::get_task))
        .route("/tasks/:id", put(handlers::update_task))
        .route("/tasks/:id", delete(handlers::delete_task))
        .with_state(pool); // this makes `State<SqlitePool>` available in every handler

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("failed to bind to port 3000");

    println!("listening on http://0.0.0.0:3000");
    axum::serve(listener, app).await.expect("server error");
}
