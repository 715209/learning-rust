use std::io;

fn main() {
    println!("Please enter the nth fibonacci you want.");

    let mut nth_fibonacci = String::new();

    io::stdin()
        .read_line(&mut nth_fibonacci)
        .expect("Failed to read line");

    let nth_fibonacci: u128 = nth_fibonacci.trim().parse().expect("Please enter a number");
    let number = generate_fibonacci(nth_fibonacci);

    println!("The {}th fibonacci number is: {}", nth_fibonacci, number);
}

// Fibonacci series = 0, 1, 1, 2, 3, 5, 8, 13, 21, 34, ......
fn generate_fibonacci(mut n: u128) -> u128 {
    let mut first = 0;
    let mut second = 1;
    let mut sum;

    while n > 0 {
        sum = first + second;
        first = second;
        second = sum;
        n -= 1;
    }

    second
}
