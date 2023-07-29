fn main() {
    let mut s = String::from("Hello world");
    let word = get_first_word(&s); // 5
    s.clear(); // s = ""; word still = 5
}


fn get_first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}