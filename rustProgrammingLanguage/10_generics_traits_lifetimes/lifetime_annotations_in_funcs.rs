fn main() {
    let s1 = String::from("long string is long");
    {
        let s2 = String::from("longer string is longer");
        let res = longest(s1.as_str(), s2.as_str());
        println!("{res} is longest!");
    }   
}


fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}