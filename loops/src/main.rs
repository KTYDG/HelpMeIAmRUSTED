fn main() {
    let mut x = 0;
    let x = loop {  // infinity shit
        println!("ALMOST INFINITY");        
        x += 1;
        if x > 5 {
            break x;
        }
    };
    println!("x = {}", x);
}
