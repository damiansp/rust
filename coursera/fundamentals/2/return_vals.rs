fn main() {
    let chunk = split_string("Hello,world".to_string(), ',', 1);
    println!("{chunk}");
}


fn split_string(s: String, delim: char, field: usize) -> String {
    let parts: Vec<&str> = s.split(delim).collect();
    let res = parts.get(field);
    res.expect("Something went wrong").to_string()
}