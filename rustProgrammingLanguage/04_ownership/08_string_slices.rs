fn main() {
    let s = String::from("hello world");
    let hello = &s[0..5];   //[..5]
    let world = &s[6..11];  //[6..]

    let first = get_first_word(&s);
    println!("First: {first}");
}


fn get_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { return &s[..i]; }
    }
    &s[..]
}