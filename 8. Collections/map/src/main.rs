use std::collections::HashMap;

fn main() {
    map();
    update_map();
}

fn update_map() {
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.chars() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    for (char, amount) in &map {
        println!("char: {}; amount: {}", char, amount);
    }
    println!();

    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    for (word, amount) in &map {
        println!("Word: {}; amount: {}", word, amount);
    }
    println!("//////////////////////////////////////////////////////////////////");
}

fn map() {
    println!("//////////////////////////////////////////////////////////////////");
    let mut marks: HashMap<String, i32> = HashMap::new();
    marks.insert("DiNYA".to_string(), 54);
    marks.insert("SoNYA".to_string(), 54);
    marks.insert("Alex".to_string(), 25);
    marks.insert("Lilia".to_string(), 45);
    for i in &marks {
        println!("Name: {}, mark: {}", i.0, i.1);
    }
    println!();

    let mark_of_alex = marks.get("Alex").copied().unwrap_or(-1);
    println!("Mark of Alex: {}", mark_of_alex);
    let mark_of_olga = marks.get("Olga").copied().unwrap_or(-1);
    if mark_of_olga == -1 {
        println!("Olga doesn't exist");
    } else {
        println!("Mark of Olga: {}", mark_of_olga);
    }
    println!();

    for (name, mark) in &marks {
        println!("Name: {}, mark: {}", name, mark);
    }
    println!();

    marks.insert("DiNYA".to_string(), 69);
    println!("Changed mark of DiNYA!");
    for (name, mark) in &marks {
        println!("Name: {}, mark: {}", name, mark);
    }
    println!();

    marks.entry("DiNYA".to_string()).or_insert(25);
    println!("Tried to readd DiNYA with different mark and nothing must be changed!");
    marks.entry("Olga".to_string()).or_insert(21);
    println!("Olga must be added!");
    for (name, mark) in &marks {
        println!("Name: {}, mark: {}", name, mark);
    }
    println!("//////////////////////////////////////////////////////////////////");
}
