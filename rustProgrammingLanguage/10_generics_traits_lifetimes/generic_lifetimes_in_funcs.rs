fn main() {
    let s1 = String::from("abcd");
    let s2 = "xyz";
    let res = longest(s1.as_str(), s2);
    println!("The longest string is {res}");
}


fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}