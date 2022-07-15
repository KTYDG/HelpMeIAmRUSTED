use std::io::{self, Write};

fn main() {
    println!("//////////////////////////////////////////////////////////////////");
    string_and_str();
    push();
    plus();
    index();
}

fn index() {
    for i in "Здравствуйте".chars() {
        print!("char: {i}\n");
        io::stdout().flush().unwrap();
    }
    println!();
    for i in "Здравствуйте".bytes() {
        println!("bytes: {i}");
    }
    println!("//////////////////////////////////////////////////////////////////");
}

fn plus() {
    let first = "1 + ".to_string();
    println!("first: {first}");
    let second = "2 = 3".to_string();
    println!("second: {}", second);
    let complete = first + &second;
    println!("complete: {complete}");
    println!();

    let tic = String::from("tic");
    let toe = String::from("toe");
    let tic_tac_toe = tic + "-" + "tac" + "-" + &toe;
    println!("tic-tac-toe: {}", tic_tac_toe);
    // println!("Can't print tic! {}", tic);
    println!();

    let tic = String::from("tic");
    let toe = String::from("toe");
    let tic_tac_toe = format!("{}-tic-{}", tic, toe);
    println!("tic-tac-toe: {}", tic_tac_toe);
    println!("Can print tic! {}", tic);
    println!("//////////////////////////////////////////////////////////////////");
}

fn push() {
    let mut s = "I_love".to_string();
    println!("Before push: {s}");
    s.push('_');
    println!("After push: {s}");
    s.push_str("Rust.");
    println!("After push_str: {s}");
    let addition = " It's so cool!";
    s.push_str(addition);
    println!("Addition: {addition}");
    println!("After second push_str: {s}");
    println!("//////////////////////////////////////////////////////////////////");
}

fn string_and_str() {
    let string_str = "this is string literal";
    println!("1: {}", string_str);
    println!();

    let string_string = string_str.to_string();
    println!("2: {}", string_string);
    let string_string = "this is String".to_string();
    println!("3: {}", string_string);
    println!();

    let string_string = String::from(string_str);
    println!("4: {}", string_string);
    let string_string = String::from("this is String");
    println!("5: {}", string_string);
    println!();

    println!("For test, what nothing borrowed: {}", string_str);
    println!("//////////////////////////////////////////////////////////////////");
}
