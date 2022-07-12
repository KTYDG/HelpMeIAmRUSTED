use std::io::stdin;
#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

fn main() {
    let r = rectangle();
    let r2 = Rectangle::create(10, 5);
    let r3 = Rectangle::create(11, 6);
    println!("The area of rectangle {}x{} is: {}", r.height, r.width, r.area());
    println!("Can r2 hold r3? Answer: {} ", r2.can_hold(&r3));
    println!("Can r3 hold r2? Answer: {} ", r3.can_hold(&r2));
    // println!("{:#?}", r);
    // dbg!(&r);
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, this: &Rectangle) -> bool {
        self.height > this.height && self.width > this.width
    }

    fn create(height: u32, width: u32) -> Self {
        Self { 
            height,
            width,
        }
    }
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
        // height: dbg!(height),
        height,
        width,
    }
}
