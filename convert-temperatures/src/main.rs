use std::io;

fn main() {
    println!("Enter your temperature like this: 10 C or 10 F.");

    let mut temperature = String::new();

    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read line");

    let temperature: Vec<_> = temperature.trim().split_whitespace().collect();
    let number: f32 = temperature[0].parse().expect("Please enter a number");

    match temperature[1] {
        "c" | "C" => println!("{} °C is {} °F", number, number * 9.0 / 5.0 + 32.0),
        "f" | "F" => println!("{} °F is {} °C", number, (number - 32.0) * 5.0 / 9.0),
        _ => panic!("Invalid symbol"),
    };
}
