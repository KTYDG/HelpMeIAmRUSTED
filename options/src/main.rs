//#![allow(dead_code, unused)]
//#[derive(Debug)]


// enum UsState {
//     Alabama,
//     Alaska,
//     LA,
// }

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
// }

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => {
//             println!("Lucky penny!");
//             1
//         }
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("State quarter from {:?}", state);
//             25
//         }
//     }
// }

// fn main() {
//     value_in_cents(Coin::Quarter(UsState::Alaska));
// }

fn main() {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    println!("{:?}", y);
    let sum = x + get(y);
    println!("Sum = {sum}");
}

fn get(x:Option<i8>) -> i8 {
    match x {
        None => 0,
        Some(i) => i,
    }
}
