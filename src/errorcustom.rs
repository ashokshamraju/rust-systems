use std::{fmt::self, write};

#[derive(Debug)]
enum AppError {
    DivideByZero,
    ParseFailed(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::DivideByZero=> write!(f, "Divide by Zero Error"),
            AppError::ParseFailed(msg)=> write!(f, "Parse Error: {}", msg)
        }
    }
}

impl std::error::Error for AppError {}

fn compute(a:&str, b:&str)->Result<f64, AppError>{
    
    let parse_num_a = a.parse::<f64>()
                .map_err(|_| AppError::ParseFailed(format!("could not parse '{}'", a)))?;
    
    let parse_num_b = b.parse::<f64>()
                .map_err(|_| AppError::ParseFailed(format!("could not parse '{}'", a)))?;
    
    if parse_num_b == 0.0 {
        //Err(AppError::ParseFailed(parse_num_b.to_string()))
        Err(AppError::DivideByZero)
    }else {
        let result = parse_num_a / parse_num_b;

        Ok(result)
    }
}


fn main() {
    let cases = [("10","2"),("5", "0"),("abc","3")];

    for (a,b) in cases  {
        match compute(a,b) {
            Ok(result)=>println!("{} / {} = {}", a, b, result),
            Err(e) => println!("Error: {}", e),
        }
    }

    let err = AppError::ParseFailed("could not parse abc".to_string());

    // println! — writes straight to stdout
    println!("{}", err);

    // format! — builds a new String, doesn't print anything itself
    let msg: String = format!("Error occurred: {}", err);
    println!("{}", msg); // now we print it ourselves

    // .to_string() — also builds a new String, via the blanket ToString impl
    let msg2: String = err.to_string();
    println!("{}", msg2);

    // Writing into an existing String buffer directly with write!
    use std::fmt::Write; // note: this is the fmt::Write trait, not io::Write
    let mut buffer = String::from("Log: ");
    write!(buffer, "{}", err).unwrap(); // appends into the existing buffer
    println!("{}", buffer);
}