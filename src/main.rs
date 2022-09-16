use std::io;

fn main() {

    println!("Enter your weight: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let weight: f32 = input.trim().parse().unwrap();
    println!("Weight: {}", weight);


    let mut weight_on_mars :f32 = calculate_weight_on_mars(weight);
    weight_on_mars *= 1000.0;
    println!("Weight on Mards: {}g", weight_on_mars);
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711 
}
