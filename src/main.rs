use std::io;

fn main() {
    println!("Print temperature in Fahrenheit:");

    let mut temperature = String::new();

    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read line");

    let mut temperature: f32 = temperature
        .trim()
        .parse()
        .expect("Please type a number");

    // (0°F − 32) × 5/9 = -17.78°C

    temperature = fahrenheit_to_celsius(temperature);
    println!("{}°F", temperature);
}

fn fahrenheit_to_celsius (temperature: f32) -> f32 {
    (temperature - 32.0) * 5.0 / 9.0
}