use std::io;

fn main() {
    
    let mut x = String::new();

    println!("\n\n\nTriangle Maker\n\nPlease enter a number to represent the length of the longest side of the triangle.\n");

    io::stdin()
        .read_line(&mut x)
        .expect("Failed to read line!");

    let x: u32 = x.trim().parse().expect("Could not parse x!");

    let mut y: u32 = 1;

    let mut star1 = String::from("*");
    let star2 = String::from("*");

    println!("\n\n\n");

    while y <= x {
        println!("{}", star1);
        star1.push_str(&star2);
        y+=1;
    }

    println!("\n\n\n");

}
