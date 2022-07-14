use crate::garden::vegetables::{Asparagus, Nya, Rose};

mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
    let plant = Rose {};
    println!("I'm growing {:?}!", plant);
    let plant = Nya {};
    println!("I'm growing {:?}!", plant);
}
