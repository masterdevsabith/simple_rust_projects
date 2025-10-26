use std::io;

fn main() {
    let mut temperature = String::new();
    let mut mode = String::new();

    println!("mode ? (celsius/fahrenheit)");
    io::stdin()
        .read_line(&mut mode)
        .expect("failed to read mode line");

    let mode = mode.trim();

    if mode == "celsius" {
        println!("Enter temperature in Celsius");

        io::stdin()
            .read_line(&mut temperature)
            .expect("failed to read temperature line");

        let temperature: f32 = temperature.trim().parse().expect("couldn't parse");
        let celsius_to_fahrenheit: i32 = celsius_to_fahrenheit(temperature) as i32;

        println!("{temperature} celsius to fahrenheit is {celsius_to_fahrenheit}°F");
    } else if mode == "fahrenheit" {
        println!("Enter temperature in fahrenheit");

        io::stdin()
            .read_line(&mut temperature)
            .expect("failed to read temperature line");

        let temperature: f32 = temperature.trim().parse().expect("couldn't parse");
        let fahrenheit_to_celsius:i32 = fahrenheit_to_celsius(temperature) as i32;

        println!("{temperature} celsius to fahrenheit is {fahrenheit_to_celsius}°C");
    }
}

fn celsius_to_fahrenheit(celsius: f32) -> f32 {
    celsius * (9.0 / 5.0) + 32.0
}
fn fahrenheit_to_celsius(fahrenheit: f32) -> f32 {
    (fahrenheit - 32.0) * (5.0 / 9.0)
}
