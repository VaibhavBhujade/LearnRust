use std::io;

fn main() {
    println!("Enter N for nth fibonacci number");

    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Unable to read line");

    let n = n
        .trim()
        .parse()
        .expect("Enter a valid number");

    let nth=fibo(n);

    println!("Nth Fibonacci number is {nth}");
}

fn fibo(n:i32) -> i32 {

    if n==1 {
        return 0;
    }

    if n==2 {
        return 1;
    }

    fibo(n-1)+fibo(n-2)
}