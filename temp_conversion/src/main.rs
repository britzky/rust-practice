fn main() {
    println!("Hello, world!");
    let celsius = fahrenheit_to_celsius(70.0);
    println!("The temprature in celsius is {celsius} degrees")
}

fn fahrenheit_to_celsius(x: f64) -> f64 {
    println!("The temprature in fahrenheit is {x} degrees");
    (x - 32.0) * 5.0 / 9.0
}

