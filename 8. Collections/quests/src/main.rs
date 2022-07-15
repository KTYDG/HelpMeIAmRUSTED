use quest_1::quest;
use quest_2::quest2;

fn main() {
    quest();
    quest2();
}

mod quest_1;
mod quest_2 {
    use std::io;

    pub fn quest2() {
        println!("Enter the string to be Pig-latinized");

        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read from stdin");

        let mut pig_latin = String::new();
        
        for word in input.split_whitespace() {
            let char = word.chars().next().unwrap();
            match char {
                'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => {
                    pig_latin += &format!("{}-hay ", word);
                }
                _ => {
                    pig_latin += &format!("{}-{}ay ", &word[1..], char);
                }
            }
        }

        println!("Pig: {}", pig_latin);
    }
}