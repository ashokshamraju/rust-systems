use task_notes::{app_router, db};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init(); //logging system

    let pool = db::init_pool("sqlite::memory:")
                .await
                .expect("Failed to initialize database");
    
    let app = app_router(pool);


    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("failed to bind to port");

    println!("listening on http://0.0.0.0.3000");
    axum::serve(listener, app).await.expect("Server Error");
}
