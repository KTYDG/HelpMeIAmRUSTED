use std::collections::HashMap;

use crate::quest_1::tools::print_v;

pub fn average(v:&Vec<i32>) {
    let mut sum = 0;
    for i in v {
        sum += i;
    }
    let len:i32 = v.len().try_into().unwrap();
    println!("Average = {}", sum/len);
}
pub fn median(v:&mut Vec<i32>) {
    v.sort();
    println!("Sorted:");
    print_v(v);
    println!("Mediana = {}", v[v.len()/2]);
}
pub fn mode_of_v(v:&Vec<i32>) {
    let mut mode:HashMap<i32, i32> = HashMap::new();
    for i in v {
        let count = mode.entry(*i).or_insert(0);
        *count += 1;
    }
    let mut max = 0;
    let mut rem: i32 = 0;
    for (num, amount) in &mode {
        if *amount >= max {
            max = *amount;
            rem = *num;
        }
    }
    println!("Mode of v: {}; amount: {}", mode.get_key_value(&rem).unwrap().0, mode.get_key_value(&rem).unwrap().1,)
}
