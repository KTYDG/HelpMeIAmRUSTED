use rand::{thread_rng, Rng};

pub fn create() -> Vec<i32> {
    let mut v = Vec::new();
    for _ in 1..15 {
        v.push(thread_rng().gen_range(1..6));
    }
    v
}
pub fn print_v(v:&Vec<i32>) {
    for i in v {
        println!("v: {}", i);
    }
}
