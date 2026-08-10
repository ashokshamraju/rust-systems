use std::println;

struct Counter {
    count: u32,
    max: u32
}

impl Counter {
    fn new(max:u32)->Counter {
        Counter { count: 0, max }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }        
    }
}

fn main() {
    let counter = Counter::new(5);

    for n in counter {
        println!("{}",n);
    }

    let counter2 = Counter::new(5);

    let sum:u32 = counter2.map(|x|x * 2).filter(|x| x % 3 == 0).sum();

    println!("{}",sum);
}