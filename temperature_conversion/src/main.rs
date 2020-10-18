use std::io;

/// (x °F − 32) × 5/9 = x °C
fn convert_to_celcius(temperature: i32) -> i32 {
    println!("celcius");
    (temperature - 32) * (5 / 9)
}
/// (0 °C × 9/5) + 32 = 32 °F
fn convert_to_fareiheint(temperature: i32) -> i32 {
    (temperature * (9 / 5)) + 32
}

/// Function to convert fareinheit temperature to celcius and vice-versa using the formula (x °F − 32) × 5/9 = x °C
/// ### Arguments
/// * `temperature`: i32 - temperature to be converted
/// * `temp_unit`: char - temperature unit
fn convert_teperature(temperature: i32, temp_unit: char) -> i32 {
    if temp_unit == 'c' {
        convert_to_fareiheint(temperature)
    } else {
        convert_to_celcius(temperature)
    }
}

fn main() {
    let mut temperature = String::new();
    let mut unit = String::new();

    println!("Insert temperature");

    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read Line");
    println!("Insert unit");

    io::stdin()
        .read_line(&mut unit)
        .expect("Failed to read Line");

    let temperature: i32 = temperature.trim().parse().expect("Not a number");

    let unit: char = unit.trim().parse().expect("Not a char");

    println!(
        "converted temperature {}",
        convert_teperature(temperature, unit)
    );
}
