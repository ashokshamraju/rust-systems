use std::println;

struct Stack<T> {
    items:Vec<T>,
}

impl <T> Stack<T> {
    fn new()-> Self{
        Self { items: Vec::new() }
    }

    fn push(&mut self, item:T) {
        self.items.push(item);
    }

    fn pop(&mut self)->Option<T> {
        self.items.pop()        
    }

    fn peek(&self)->Option<&T> {
        self.items.last()
    }

    fn is_empty(&self)->bool {
        self.items.is_empty()
    }
}

fn main() {
    let mut stack:Stack<i32> = Stack::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    println!("Peek {:?}",stack.peek());
    println!("Pop: {:?}",stack.pop());
    println!("Pop: {:?}",stack.pop());
    println!("Is Empty: {:?}",stack.is_empty());
    println!("Pop: {:?}",stack.pop());
    println!("Is Empty: {:?}",stack.is_empty());

    let mut stack2: Stack<String> = Stack::new();
    stack2.push("hello".to_string());
    stack2.push("world".to_string());
    println!("Pop: {:?}", stack2.pop());
}