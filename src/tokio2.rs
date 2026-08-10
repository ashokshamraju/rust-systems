// use tokio::time::{sleep, Duration};
// use std::{println, sync::{Arc,Mutex}, vec};

// async fn increment(counter:Arc<Mutex<i32>>) {
//     let mut num = counter.lock().unwrap();

//     *num += 1;
//     sleep(Duration::from_millis(100)).await; // hold the lock across
//     println!("Done!");
// }
// #[tokio::main]

// async fn main() {
//     let counter = Arc::new(Mutex::new(0));

//     let mut handles = vec![];

//     for _ in 0..3 {
//         let c = Arc::clone(&counter);
//         handles.push(tokio::spawn(increment(c)));
//     }

//     for h in handles{
//         h.await.unwrap();
//     }

//     println!("Final: {}", *counter.lock().unwrap());
// }

// THE ABOVE CODE WILL NOT COMPILE BECAUSE MUTEX GUARD IS NOT 'SEND'
// AFTER MUTEX LOCK, THREAD GOES INTO SLEEP ANS WHEN IT WAKES UP
//IT IS NOT GUARANTTED THAT THE IT RUNS ON THE SAME THREAD. IT MIGHT
//BE RUNNING ON THREAD A AND WHEN IT RESUMES IT MIGHT BE RUNNING ON
//THREAD B. SO THREAD B TRIES TO UNLOCK MUTEX WHICH IT NEVER ACQUIRED


//*********FIX 1 */
// use tokio::time::{sleep, Duration};
// use std::{println, sync::{Arc,Mutex}, vec};

// async fn increment(counter:Arc<Mutex<i32>>) {
//     {
//         let mut num = counter.lock().unwrap();

//         *num += 1;
//     }// LOCK RELEASED HERE
//     sleep(Duration::from_millis(100)).await; // hold the lock across
//     println!("Done!");
// }
// #[tokio::main]

// async fn main() {
//     let counter = Arc::new(Mutex::new(0));

//     let mut handles = vec![];

//     for _ in 0..3 {
//         let c = Arc::clone(&counter);
//         handles.push(tokio::spawn(increment(c)));
//     }

//     for h in handles{
//         h.await.unwrap();
//     }

//     println!("Final: {}", *counter.lock().unwrap());
// }


//*********FIX 2 */
// THIS FIX USES MUTEX FROM TOKIO INSTEAD OF STD
// TOKIO MUTEX RETURNS MUTEXGUARD DIRECTLY AND HENCE USE AWAIT
// STD MUTEX RETURNS RESULT AND HENCE USE UNWRAP
use tokio::time::{sleep, Duration};
use tokio::sync::Mutex;
use std::{println, sync::Arc, vec};

async fn increment(counter:Arc<Mutex<i32>>) {
    
    let mut num = counter.lock().await;

    *num += 1;
    sleep(Duration::from_millis(100)).await; // hold the lock across
    println!("Done!");
}
#[tokio::main]

async fn main() {
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..3 {
        let c = Arc::clone(&counter);
        handles.push(tokio::spawn(increment(c)));
    }

    for h in handles{
        h.await.unwrap();
    }

    println!("Final: {}", *counter.lock().await);
}