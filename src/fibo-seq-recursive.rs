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
    println!("To quit the program, type 'exit'.");

    loop {
        let mut s = String::new();

        io::stdin()
            .read_line(&mut s)
            .expect("Failed to read line.");

        if s.trim() == "exit" {
            break;
        }

        let n: u32 = match s.trim().parse() {
            Ok(n) => n,
            Err(_) => continue,
        };

        println!("The {}-th fibonacci is: {}", n, fib(n));
    }
}
