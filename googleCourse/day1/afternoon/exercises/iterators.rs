fn main() {
    let v: Vec<i8> = vec![10, 20, 30];
    let mut iter = v.iter();
    let v0: Option<&i8> = iter.next();
    println!("v0: {v0:?}");
    println!("v[0]: {:?}", iter.next());
    println!("v[1]: {:?}", iter.next());
    println!("v[2]: {:?}", iter.next());
    println!("All gone: {:?}", iter.next());

    let v: Vec<String> = vec![String::from("foo"), String::from("bar")];
    let mut iter = v.into_iter();
    let v0: Option<String> = iter.next();
    println!("v0: {v0:?}");

    let v: Vec<String> = vec![String::from("foo"), String::from("bar")];
    for word in &v {
        println!("word: {word}");
    }
    for word in v {
        println!("word: {word}");
    }
}

pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}


pub trait IntoIterator {
    type Item;
    type IntoIter: Iterator<Item=Self::Item>;

    fn into_iter(self) -> Self::IntoIter;
}