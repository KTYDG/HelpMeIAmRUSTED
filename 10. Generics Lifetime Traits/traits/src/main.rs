use std::{fmt::Display};

fn main() {
    let tw = Tweet {
        username: String::from("Tester"),
        content: String::from("This is test of traits."),
        reply: false,
        retweet: false,
    };
    let na = NewsArticle {
        headline: String::from("OH, THIS IS TEST!"),
        location: String::from("Testeross"),
        author: String::from("Tester"),
        content: String::from("So, looking for traits."),
    };
    let bl = Blank {
        blank: "Empty".to_string(),
    };
    println!("{}", tw.summarize());
    println!("{}", na.summarize());
    println!("{}", bl.summarize());
    println!("///////////////////////////////////////////////////////////////////////////");
    notify(&tw);
    println!();
    notify_double(&tw, &bl);
    println!();
    notify_double_same(&bl, &bl);
    notify_display(&5);
    some_function(&32, &32);
}

fn _returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

// Пример как удобно установить сразу НЕСКОЛЬКО границ типажа,
// чтоб это выглядело не убого в строку
fn some_function<T, U>(_t: &T, _u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + std::fmt::Debug,
{
    0
}

pub trait Summary {
    fn summarize(&self) -> String {
        format!("Find more here {}", self.summarize_author())
    }
    fn summarize_author(&self) -> String;
}
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
pub fn notify_double(item1: &impl Summary, item2: &impl Summary) {
    println!("FIRST Breaking news! {}", item1.summarize());
    println!("SECOND Breaking news! {}", item2.summarize());
}
pub fn notify_double_same<T: Summary>(item1: &T, item2: &T) {
    println!("FIRST Breaking news! {}", item1.summarize());
    println!("SECOND Breaking news! {}", item2.summarize());
}
pub fn notify_display(item: &(impl Summary + Display)) {
    println!("Breaking news! {}", item.summarize());
}

impl Summary for i32 {
    fn summarize(&self) -> String {
        self.to_string()
    }
    fn summarize_author(&self) -> String {
        self.to_string()
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub struct Blank {
    pub blank: String,
}

impl Summary for Blank {
    fn summarize_author(&self) -> String {
        format!("@{}", self.blank)
    }
}
