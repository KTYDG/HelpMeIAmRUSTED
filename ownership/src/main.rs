fn main() {
    let s = String::from("hello");

    let slice = &s[0..2];
    print!("{slice}\n");
    let slice = &s[..2];
    print!("{slice}\n");
    let slice = &s[..];
    print!("{slice}\n");
    let slice = &s[0..];
    print!("{slice}\n");
    let slice = &s[2..];
    print!("{slice}\n");
}

