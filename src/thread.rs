use std::{ println, thread, vec};

fn main() {
    let numbers = vec![1,2,3,4,5,6,7,8,9,10];

    let chunck1 = numbers[0..5].to_vec();
    let chunk2 = numbers[5..10].to_vec();

    let t1 = thread::spawn(move ||{
        let sum:i32 = chunck1.iter().sum();

        sum
    });

    let t2 = thread::spawn(move ||{
        let sum:i32 = chunk2.iter().sum();

        sum
    }) ;

    let result1 = t1.join().unwrap();
    let result2 = t2.join().unwrap();

    let total = result1 + result2;

    println!("Total sum is {}",total);

    for i in numbers{
        println!("{}",i);
    }
}