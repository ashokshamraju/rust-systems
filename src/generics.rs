use std::ops::Index;

fn largest<T: PartialOrd>(list: &[T])->Option<&T> {
    
    if list.is_empty() {
        return  None;
    }
    
    let mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    Some(largest)

}

fn largest_new<T: PartialOrd>(list: &[T])->&T {
    
    list.iter().fold(&list[0],|largest: &T, x| {
        if x > largest {
            x
        }else {
            largest
        }
    })
}

fn main() {
    let numbers = vec![34, 50, 25, 100, 65];
    println!("Largest number: {}", largest(&numbers).unwrap());
    println!("Largest number: {}", largest_new(&numbers));

    let chars = vec!['y', 'm', 'a', 'q'];
    println!("Largest char: {}", largest(&chars).unwrap());
    println!("Largest char: {}", largest_new(&chars));

    let words = vec!["apple".to_string(), "zebra".to_string(), "mango".to_string()];
    println!("Largest word: {}", largest(&words).unwrap());
    println!("Largest word: {}", largest_new(&words));
}