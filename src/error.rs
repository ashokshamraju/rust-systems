use std::{println, vec};

fn divide(a: f64, b: f64)->Result<f64, String> {
    if b == 0.0 {
        Err("Not valid".to_string())
    }else {
        let result = a / b;

        Ok(result)
    }
}

fn parse_and_sum(items: Vec<&str>)->Result<i32, std::num::ParseIntError> {
    
    let mut sum = 0;
    for element in items {
        let parsed_num = element.parse::<i32>()?;

        sum += parsed_num;
    }
    Ok(sum)
}
fn parse_and_sum_iter(items: Vec<&str>)->Result<i32, std::num::ParseIntError> {
    items.iter()
        .map(|s|s.parse::<i32>())
        .sum()
}

fn main() {
    let cases = [(10.0,2.0),(5.0, 0.0), (9.0,3.0)];

    for (a,b) in cases {
        match divide(a, b) {
            Ok(result) =>println!("{} / {} = {}", a, b, result),
            Err(e)=>println!("Error: {}", e),
        }
    }

    match parse_and_sum(vec!["10","20","30"]) {
        Ok(sum)=> println!("Parse for loop Sum: {}",sum),
        Err(e)=> println!("Parse Error: {}",e),
    }

    match parse_and_sum(vec!["10","abc","30"]) {
        Ok(sum)=> println!("Parse for loop Sum: {}",sum),
        Err(e)=> println!("Parse Error: {}",e),
    }

    match parse_and_sum_iter(vec!["10","20","30"]) {
        Ok(sum)=> println!("Iter Sum: {}",sum),
        Err(e)=> println!("Parse Error: {}",e),
    }

    match parse_and_sum_iter(vec!["10","abc","30"]) {
        Ok(sum)=> println!("Iter Sum: {}",sum),
        Err(e)=> println!("Parse Error: {}",e),
    }
}