use std::io;

fn main() {

    let mut input = String::new();

    println!("Enter weight on Earth to convert for weight on Mars: ");
    io::stdin().read_line(&mut input).unwrap();

    let weight_as_float: f32 = input.trim().parse().unwrap();

    let mut result: f32 = convert_to_mars_weight(weight_as_float);

    result = result * 1000.0;

    println!("Weight on mars is {} g", result);
}


fn convert_to_mars_weight(_weight:f32 ) -> f32 {
    (_weight/9.81) * 3.711
}