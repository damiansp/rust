//--------//--------//--------//--------//--------//--------//--------//-------
fn main() {
    let article = NewsArticle{
        headline: String::from("Penguins win Stanley Cup!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Hockey Nut"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best team in the NHL.")
    };
    println!("New article: {}", article.summarize());

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people..."),
        is_reply: false,
        is_retweet: false
    };
    println!("1 new tweet: {}", tweet.summarize());
}


pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        // default behavior
        format!("(Read more from {}...)", self.summarize_author())
    }
}


pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String
}


impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("by {}", self.author)
    }

    fn summarize(&self) -> String {
        format!(
            "{}, {} ({})",
             self.headline,
              self.summarize_author(),
              self.location)
    }
}


pub struct Tweet {
    pub username: String,
    pub content: String,
    pub is_reply: bool,
    pub is_retweet: bool
}


impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    fn summarize(&self) -> String {
        format!("{}: {}", self.summarize_author(), self.content)
    }
}


pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}


fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from("Penguins win the Stanley Cup!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Joe Hockeyman"),
            content: String::from("Once again the Penguins amaze")
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people"),
            reply: false,
            retweet: false
        }
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
