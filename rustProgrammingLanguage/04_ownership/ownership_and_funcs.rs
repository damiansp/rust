fn main() {
    let s1 = give_ownership();
    let s2 = String::from("hello");
    let (s2, len) = calculate_len(s2);
    println!("the length of {s2} is {len}");
    let len2 = calc_len(&s2);
    println!("the length of {s2} is {len2}");
    let s3 = take_and_return(s2); // s2 out of scope here

}  // s1 and s3 go out of scope here


fn give_ownership() -> String {
    let some_str = String::from("yours");
    some_str
}


fn calculate_len(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}


fn calc_len(s: &String) -> usize {
    s.len()
}


fn take_and_return(s: String) -> String {
    s
}