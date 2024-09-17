use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(& mut input).unwrap();
    let weight: f32 = input.trim().parse().unwrap();
    let mars_weight = calculate_on_mars(weight);
    println!("Hello, world! {} kg on Earth is {} kg on Mars", 100.0, mars_weight);
}

fn calculate_on_mars(weight: f32) -> f32 {
    (weight / 9.81 ) * 3.711
}