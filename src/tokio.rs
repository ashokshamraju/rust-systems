use tokio::time::{Duration, sleep};

async fn task(name:&str, secs:f64) {
    println!("{} Starting...",name);
    sleep(Duration::from_secs(secs as u64)).await;
    println!("{} Done!",name);
}

#[tokio::main]
async  fn main() {
    // 1. Runs in sequence. Not truly concurrent
    // task("A", 2.0).await;
    // task("B", 2.0).await;

    //2. runs concurrently
    //tokio::join!(task("A", 2.0),task("B", 2.0));

    //3. runs concurrently. await is needed on handlers so that main thread
    //knows when the handlers finished or waiting for it's result
    let h1 = tokio::spawn(task("A", 2.0));
    let h2 = tokio::spawn(task("B", 2.0));

    h1.await.unwrap();
    h2.await.unwrap();
}