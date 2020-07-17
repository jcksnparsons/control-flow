fn main() {
    println!("{}", fibonacci(40));
}

// fn farenheit_to_celsius(x: i32) -> i32 {
//     let celsius = ((x as f64) - 32.0) * 0.556;
//     celsius as i32
// }


// fn celsius_to_farenheit(x: i32) -> i32 {
//     let farenheit = ((x as f64) * 1.8) + 32.0;
//     farenheit as i32
// }

fn fibonacci(x: u32) -> u32 {
    match x {
        0 => 1,
        1 => 1,
        _ => fibonacci(x - 1) + fibonacci(x - 2)
    }
}