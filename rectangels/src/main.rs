use std::io::stdin;
#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

fn main() {
    let r = rectangle();
    println!("The area of rectangle {}x{} is: {}", r.height, r.width, area(&r));
    println!("{:#?}", r);
    dbg!(&r);
}

fn area(r: &Rectangle) -> u32 {
    r.height * r.width
}

fn rectangle() -> Rectangle {
    println!("Put height: ");
    let height = loop {
        let mut height = String::new();
        stdin().read_line(&mut height).expect("smth wrong...");
        match height.trim().parse() {
            Ok(h) => {
                break h;
            }
            Err(_) => {
                println!("Try again: ");
                height.clear();
                continue;
            }
        };
    };
    println!("Put width: ");
    let width = loop {
        let mut width = String::new();
        stdin().read_line(&mut width).expect("smth wrong...");
        match width.trim().parse() {
            Ok(w) => {
                break w;
            }
            Err(_) => {
                println!("Try again: ");
                width.clear();
                continue;
            }
        };
    };

    Rectangle {
        height: dbg!(height),
        width,
    }
}
