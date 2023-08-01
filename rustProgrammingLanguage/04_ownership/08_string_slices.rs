fn main() {
    let s = String::from("hello world");
    let hello = &s[0..5];   //[..5]
    let world = &s[6..11];  //[6..]

    let first = get_first_word(&s);
    println!("First: {first}");

    let mut t = String::from("Hello world");
    let word = get_first_word(&t);
    //t.clear(); // error -- needed for next
    println!("First word is {word}")

    let u = "Hello, World!";  // type &str (not String)
}


fn get_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { return &s[..i]; }
    }
    &s[..]
}