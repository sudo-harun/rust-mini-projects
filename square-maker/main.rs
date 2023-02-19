use std::io;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let mut size = String::new();

    println!("\n\n\nSquare maker 1.0\n");
    println!("This program takes a width and height from you, makes a rectangle, and returns the area of that rectangle.");
    println!("\nPlease enter a width.\n");

    io::stdin()
        .read_line(&mut size)
        .expect("Failed to read line.");

    let size: u32 = size.trim().parse().expect("Failed to parse.");

    let sq1 = Rectangle::square(size);
    println!("\n\nA square with a width of {} and a height of {} was successfully created.", &size, &size);
    println!("The area of that square is {}.\n\n\n", sq1.area());
}
