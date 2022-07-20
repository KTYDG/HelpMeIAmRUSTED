fn main() {
    lifetime1();
    println!();

    lifetime2();
    println!();

    println!("S: {}", first_rule("test"));
    println!("I: {}", first_rule_2(&65));
    println!();

    static_lifetime();
    println!();

    lifetime3();
}

fn lifetime3() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result =
        longest_with_an_announcement(string1.as_str(), string2, "Let's find out the longest!");
    println!("The longest string is {}", result);
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: std::fmt::Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn static_lifetime() {
    let st: &'static str = "I am static str which lives entire program time";
    println!("{}", st);
}

fn first_rule(s: &str) -> &str {
    s
}
fn first_rule_2(i: &i32) -> &i32 {
    i
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn lifetime2() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    println!("novel: {}", i.part);
    i.announce_and_return_part("Does it work?");
    println!("level: {}", i.level());
}

fn lifetime1() {
    let mut string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
    string1 = String::from("VERY LONG SHIT");
    println!("{}", string1);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
// fn long<'a>(x: &str, y: &str) -> &'a str {
//     static result: String = "really long string".to_string();
//     result.as_str()
// }
