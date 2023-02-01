use std::io;

fn main() {
    println!("\n\nTemperature Converter\n");

    let mut temp = String::new();

    let mut input = String::new();

    println!("Please enter a temperature value (no letters!!!)\n");

    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");

    let temp: i32 = temp.trim().parse().expect("Failed to convert");

    println!("\nType 'f' and hit enter to convert to Fahrenheit.");
    println!("Type 'c' and hit enter to convert to Celsius.\n");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");

    let input = input.trim();

    if input == "f" {
        c_to_f(temp);
    } else if input == "c" {
        f_to_c(temp);
    }
}

fn c_to_f(temp: i32) {

    let _new_temp: i32 = temp * 9/5;
    let _new_temp: i32 = _new_temp + 32;

    println!("\n{temp} converted to Fahrenheit is: {_new_temp}\n\n");
}

fn f_to_c(temp: i32) {
    let _new_temp: i32 = temp - 32;
    let _new_temp: i32 = _new_temp * 5/9;

    println!("\n{temp} converted to Celsius is: {_new_temp}\n\n");
}