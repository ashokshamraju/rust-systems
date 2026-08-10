
use std::{format, println};

trait Greet {
    fn name(&self)->String;

    fn greet(&self)->String {
        format!("Hello {}!",self.name())
    }
}

struct English;
struct French;

impl Greet for English {
    fn name(&self)->String {
        "English".to_string()
    }
}

impl Greet for French {
    fn name(&self)->String {
        "French".to_string()
    }

    fn greet(&self)->String {
        format!("Bonjour {}",self.name())
    }
}

fn main() {
    println!("Name is {}", English.name());
    println!("Greeting is {}", English.greet());

    println!("Name is {}", French.name());
    println!("Greeting is {}", French.greet());
}