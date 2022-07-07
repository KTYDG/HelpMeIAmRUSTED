fn main() {
    let mut x: i32 = 0;
    let x: i32 = loop {
        // infinity shit
        println!("ALMOST INFINITY");
        x += 1;
        if x > 5 {
            break x;
        }
    };
    println!("x = {}", x);
    test();
    test2();
    bad_for();
    good_for();
}
fn test() {
    let mut count = 0;
    'count: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 5 {
                break;
            }
            if count == 3 {
                break 'count;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}
fn test2() {
    let mut number = 3;

    while number > 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
fn bad_for() {
    let a = [10, 228, 30, 40, 69];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}
fn good_for() {
    let a = [10, 20, 30, 40, 50, 5, 4, 3, 2, 1];

    for element in 0..a.len() {
        println!("the index is: {element}");
    }
    for element in a {
        println!("the a is: {element}");
    }
}

