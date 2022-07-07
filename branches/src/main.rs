fn main() {
    let seven = 8;
    if seven < 7 {
        println!("six");
    } else if seven > 7 {
        println!("eight");
    } else {
        println!("seven");
    }
    match seven {
        6 => println!("six"),
        7 => println!("seven"),
        8 => println!("eight"),
        i32::MIN..=5_i32 | 9_i32..=i32::MAX => println!("hehe")
    };
}
