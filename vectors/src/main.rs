// #[derive(Debug)]
struct User {
    id: u32,
    username: String,
}
fn main() {
    struct_vector();
    int_vector();
}

fn int_vector() {
    let mut v = vec![1, 2, 3, 4, 5];
    println!("\nBefore");
        for i in &v {
        println!("val: {}", i);
    }
    println!("After");
    for i in &mut v {
        *i += 10;
        println!("val: {}", i);
    }
}

fn struct_vector() {
    // Этот вектор НЕизменяемый, в него НЕЛЬЗЯ пушить
    let v2 = vec![
        User { id: 1, username: String::from("DiNYA"), },
        User { id: 2, username: String::from("SoNYA"), },
        User { id: 3, username: String::from("NYA"), },
    ];
    // v2.push(User { id: 3, username: String::from("NYA"), });

    // Этот вектор изменяемый, в него можно пушить
    let mut v: Vec<User> = Vec::new();
    v.push(User {
        id: 1,
        username: String::from("DiNYA"),
    });
    v.push(User {
        id: 2,
        username: String::from("SoNYA"),
    });
    v.push(User {
        id: 3,
        username: String::from("NYA"),
    });

    // Вывод двух векторов в терминал
    println!();
    for item in &v {
        println!("id: {}, username: {}", item.id, item.username);
    }
    println!();
    for item in &v2 {
        println!("VEC2.  id: {}, username: {}", item.id, item.username);
    }

    // Чтение отдельных элементов вектора
    println!();
    let second = &v[1];
    println!("It must be SoNYA.  id: {}, username: {}", second.id, second.username);
    let third = &v2[2];
    println!("It must be NYA.  id: {}, username: {}", third.id, third.username);

    println!();
    let second_2 = v.get(1);
    match second_2 {
        Some(val) => println!("Test get. It must be SoNYA.  id: {}, username: {}", val.id, val.username),
        None => (println!("WRONG INDEX")),
    }
    if let Some(third_2) = v2.get(2) {
        println!("Test get. It must be NYA.  id: {}, username: {}", third_2.id, third_2.username);
    }

    println!();
    let none = v.get(15);
    match none {
        Some(val) => println!("Test get. SOMETHING WRONG. It must be SoNYA.  id: {}, username: {}", val.id, val.username),
        None => (println!("Test get. SUCCESS.")),
    }
    if let Some(none) = v2.get(10) {
        println!("Test get. SOMETHING WRONG. It must be NYA.  id: {}, username: {}", none.id, none.username);
    } else {
        println!("Test get. SUCCESS.");
    }

    // Добавим +1 к id
    println!();
    for i in &mut v {
        i.id += 1;
        println!("id: {}, username: {}", i.id, i.username);
    }
}
