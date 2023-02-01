use std::io;

fn fibo_n(n: u32) -> u32 {
    if n <= 2 {
        return n - 1;
    } else {
        return fibo_n(n - 1) + fibo_n(n - 2);
    }
}

fn main() {

    println!("Please enter the nth number for the nth in Fibonacci.\n");
    
    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");

    let n: u32 = n.trim().parse().expect("Failed to convert.");

    println!("\n{}", fibo_n(n));

}