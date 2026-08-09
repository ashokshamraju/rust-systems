use std::{println, vec};

fn swap(a: &mut i32, b: &mut i32) {
    let temp = *a;
    *a = *b;
    *b = temp;
}

fn longest_name(names: Vec<String>) ->Option<String> {
    names.into_iter().max_by_key(|name|name.len())
}

fn longest_name_by_ref(names: &Vec<String>) -> Option<&str>{
    names
        .iter()//borrow
        .max_by_key(|name|name.len())
        .map(|name| name.as_str())
}

// Enable below code to check error
// fn broken(names: Vec<String>) -> Option<&str> {
//     names.iter().max_by_key(|n| n.len()).map(|n| n.as_str())
// }

fn main() {
    let mut a = 10;
    let mut b = 20;
    println!("Values before swap are a is {} and b is {}", a,b);
    swap(&mut a, &mut b);
    println!("Values after swap are a is {} and b is {}", a,b);

    let names = vec![
                            "Bob".to_string(),
                            "Mary".to_string(),
                            "Tom".to_string(),
                            "Master".to_string()];
    
    let longest = longest_name(names);

    if let Some(value) = longest {
        println!("Longest Name is {}",value);
    } else {
        println!("Something went wrong");
    }

    let names_by_ref = vec![
                            "Bob".to_string(),
                            "Mary".to_string(),
                            "Tom".to_string(),
                            "Master".to_string()];
    
    let longest_by_ref = longest_name_by_ref(&names_by_ref);

    if let Some(name1) = longest_by_ref {
        println!("Longest name by ref is {:?}",name1);
    }else {
        println!("Something went wrong");
    }
    
}