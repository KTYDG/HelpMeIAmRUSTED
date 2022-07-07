use std::io::stdin;

fn main() {
    let x: u128 = loop {
        let mut x = String::new();
        stdin().read_line(&mut x).expect("stdin error");
        match x.trim().parse() {
            Ok(x) => {
                break x;
            }
            Err(_) => continue,
        };
    };
    let fib = fibonacci(x);
    println!("fibonacci: {}", fib);
}

fn fibonacci(x: u128) -> u128 {
    if x == 0 {
        return 0;
    } else if x < 3 {
        return 1;
    } else {
        return fibonacci(x - 1) + fibonacci(x - 2);
    }
}
