fn main() {
    let x = 4;
    print_x(x);
    let x = pow_two(x);
    print_x(x);
    let str = "wow";
    let str = str.to_owned() + "_";
    println!("{str}");
}

fn print_x(x: i32) {
    println!("x = {x}");
}

fn pow_two(x: i32) -> i32 {
    x * x
}
