fn main() {
    let s1 = String::from("hello");
    let len = get_len(&s1);
    //change(&s);
}


fn get_len(s: &String) -> usize { s.len() }


fn change(s: &String) {
    s.push_str(", world!"); // nope: cannot borrow a ref as mutable
}