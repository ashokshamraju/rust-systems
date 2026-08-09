use std::{ println, vec};

enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64,f64),
}

fn area(shape: &Shape)->f64 {
    match shape {
        Shape::Circle(radius) =>{
            (*radius * *radius) * std::f64::consts::PI
        },
        Shape::Rectangle(height,width ) =>{
            *height * *width
        },
        Shape::Triangle(base,height ) => {
            (*base * *height)/2 as f64
        },
    }
}

fn main() {
    let shapes = vec![
        Shape::Circle(3.0),
        Shape::Triangle(6.0, 2.0),
        Shape::Rectangle(4.0, 5.0),
    ];

    for shape in &shapes {
        println!("area: {:.2}", area(shape));
    }
}