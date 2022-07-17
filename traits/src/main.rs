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
    println!("{}", tw.summarize());
    println!("{}", na.summarize());
}


pub trait Summary {
    fn summarize(&self) -> String;
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
}

