use std::{ format, println, time::Duration};
use tokio::time::sleep;

#[derive(Debug, Clone)]
pub struct SensorEvent {
    pub sensor_id: u32,
    pub reading: f64,
}

pub async fn fetch_sensor_data(id: u32, time_delay:u64)-> Result<SensorEvent, String> {
    println!("[Sensor {}] Initialising low level async read...",id);

    sleep(Duration::from_millis(time_delay)).await;

    if time_delay > 2000{
        return  Err(format!("Sensor {} connection timeout", id));
    }

    Ok(SensorEvent { sensor_id: id, reading: 22.4 + id as f64 })
}

pub async fn run_event_loop() {
    println!("--- Launching Concurrent Infrastructure Async Worker Event Loop ---");

    let task1 = fetch_sensor_data(1, 1000);
    let task2 = fetch_sensor_data(2, 1500);
    let task3 = fetch_sensor_data(3, 2500);

    let (res1, res2, res3) = tokio::join!(task1,task2,task3);

    if let Ok(SensorEvent { sensor_id, reading }) =  res1{
        println!("Senor id is {} and reading is {}", sensor_id,reading)
    } else if let Err(e) = res1 {
        println!("Task 1 failed: {}", e);
    }

    if let Ok(SensorEvent { sensor_id, reading }) =  res2{
        println!("Senor id is {} and reading is {}", sensor_id,reading)
    }else if let Err(e) = res2 {
        println!("Task 2 failed: {}", e);
    }

    if let Ok(SensorEvent { sensor_id, reading }) =  res3{
        println!("Senor id is {} and reading is {}", sensor_id,reading)
    } else if let Err(e) = res3 {
        println!("Task 3 failed: {}", e);
    }
}

#[tokio::main] // Macro configuring a multi-threaded asynchronous runtime executor behind the scenes
async fn main() {
    run_event_loop().await;
}