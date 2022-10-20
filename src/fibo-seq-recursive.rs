use std::io;

fn fib(n: u32) -> u32 {
    if n == 0 {
        0
    } else if n <= 2 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

fn main() {
    let mut s = String::new();

    io::stdin()
        .read_line(&mut s)
        .expect("Failed to read line.");

    let n = s.trim().parse::<u32>().unwrap();

    println!("The {}-th fibonacci is: {}", n, fib(n));
}
