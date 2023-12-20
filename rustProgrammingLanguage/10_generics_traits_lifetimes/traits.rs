//--------//--------//--------//--------//--------//--------//--------//--------
pub trait Summary {
    fn summarize(&self) -> String;
}


pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String
}


impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}


pub struct Tweet {
    pub username: String,
    pub content: String,
    pub is_reply: bool,
    pub is_retweet: bool
}


impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}


/*  elsewhere
use aggregator::{Summary, Tweet};


fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people..."),
        is_reply: false,
        is_retweet: false
    };
    println!("1 new tweet: {}", tweet.summarize());
}

*/