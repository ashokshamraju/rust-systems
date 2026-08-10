use std::{format, println, vec};

use tokio::time::{sleep, Duration};

async fn fecth_data(id:u32)->Result<String,String> {
    sleep(Duration::from_millis(500)).await; //simulate network delay
    if id == 3 {
        Err(format!("failed to fectch id {}",id))
    }else {
        Ok(format!("data for id {}",id))
    }
}

#[tokio::main]
async fn main() {

    let mut handles = vec![];
    for i in 0..5 {
        let handle = tokio::spawn(fecth_data(i));

        handles.push(handle);
    }

    for j in handles{
        match j.await {
            Ok(inner_result)=>match inner_result {
                Ok(data)=>println!("Success: {}",data),
                Err(e)=>println!("fetch error: {}",e)
            }
            Err(e)=>println!("Panic error! {}",e),
        }
    }
}
