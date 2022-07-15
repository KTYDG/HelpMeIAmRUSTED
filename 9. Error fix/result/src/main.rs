use std::fs::File;
use std::io::{ErrorKind, self};

fn main() {
    println!("ERROR? {}", read_username_from_file().unwrap_or_default());
    try_open_file();
    read_line::test_read();
}

mod read_line {
    pub fn test_read() {
        assert_eq!(
            last_char_of_first_line("Hello, world\nHow are you today?"),
            Some('d')
        );

        assert_eq!(last_char_of_first_line(""), None);
        assert_eq!(last_char_of_first_line("\nhi"), None);
    }
    
    fn last_char_of_first_line(text: &str) -> Option<char> {
        text.lines().next()?.chars().last()
    }
}

fn try_open_file() {
    // Ver 2
    let _f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // Ver 1
    // let _greeting_file_result = match File::open("hello.txt") {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("Hello.txt") {
    //             Ok(file) => file,
    //             Err(error) => panic!("Can't create new file! {}", error),
    //         },
    //         else_errors => {
    //             panic!("Can't open file! {}", else_errors);
    //         },
    //     },
    // };
}

fn read_username_from_file() -> Result<String, io::Error> {
    // Ver 4
    std::fs::read_to_string("hello.txt")

    // Ver 3
    // let mut username = String::new();
    // File::open("Hello.txt")?.read_to_string(&mut username)?;
    // Ok(username)

    // Ver 2
    // let mut username_file = File::open("hello.txt")?;
    // let mut username = String::new();
    // username_file.read_to_string(&mut username)?;
    // Ok(username)

    // Ver 1
    // let username_file_result = File::open("hello.txt");

    // let mut username_file = match username_file_result {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };

    // let mut username = String::new();

    // match username_file.read_to_string(&mut username) {
    //     Ok(_) => Ok(username),
    //     Err(e) => Err(e),
    // }
}
