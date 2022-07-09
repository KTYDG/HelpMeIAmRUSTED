// fn first_word(s: &str) -> &str {
//     let bytes = s.as_bytes();
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }

//     &s[..]
// }

// fn main() {
//     let my_string = String::from("hello world");

//     // `first_word` works on slices of `String`s, whether partial or whole
//     let word = first_word(&my_string[0..6]);
//     println!("{word}");
//     let word = first_word(&my_string[..]);
//     println!("{word}");
//     // `first_word` also works on references to `String`s, which are equivalent
//     // to whole slices of `String`s
//     let word = first_word(&my_string);
//     println!("{word}");
//     let my_string_literal = "hello world";

//     // `first_word` works on slices of string literals, whether partial or whole
//     let word = first_word(&my_string_literal[0..6]);
//     println!("{word}");
//     let word = first_word(&my_string_literal[..]);
//     println!("A? {word}");

//     // Because string literals *are* string slices already,
//     // this works too, without the slice syntax!
//     let word = first_word(my_string_literal);
//     println!("{word}");
// }

fn main() {
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];
    for item in slice {
        print!("{item} ");
    }
    // print!("\n");
    // for item in &a[2..3] {
    //     print!("{item} ");
    // }
    assert_eq!(slice, &[2, 3]);
}

