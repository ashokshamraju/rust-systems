use std::{println, vec};

trait Speak {
    fn say(&self)->String;
}

struct Dog;
struct Cat;

impl Speak for Dog {
    fn say(&self)->String {
        "Woof".to_string()
    }
}

impl  Speak for Cat {
    fn say(&self)->String {
        "Meow".to_string()
    }
}

fn main() {
    let animals: Vec<Box<dyn Speak>> = vec![
        Box::new(Dog),
        Box::new(Cat),
    ];

    for animal in &animals {
        println!("{}", animal.say())
    }
}