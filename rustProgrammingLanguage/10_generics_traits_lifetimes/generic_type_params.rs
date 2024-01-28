//34567890//34567890//34567890//34567890//34567890//34567890//34567890//34567890
use std::fmt::Display;

fn main() {
    let s1 = String::from("long string is long");
    {
        let s2 = String::from("longer string is longer");
        let res = longest(s1.as_str(), s2.as_str());
        println!("{res} is longest!");
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("No '.' found.");
    let i = ImportantExcerpt {
        part: first_sentence
    };
}


fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}


struct ImportantExcerpt<'a> {
    part: &'a str
}

fn longest_with_an_announcement<'a, T>(
    x: &'a str, y: &'a str, ann: T
) -> &'a str where T: Display {
    println!("Announcement: {ann}");
    if x.len() > y.len() { x } else { y }
}