use std::io;

fn main() {
    println!("Enter your weight (kg): ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let weight: f32 = input.trim().parse().unwrap();
    println!("Input: {}", input);
    let mars_weight = calculate_mars_weight(weight);

    println!("Your weight on Mars is: {}kg", mars_weight);
}

fn calculate_mars_weight(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}