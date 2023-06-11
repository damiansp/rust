fn main() {
    let s1: String = String::from("Hello");
    let s2: String = s1;  // moved to s2; s1 no longer accessible
    println!("s2: {s2}");
    //println!("s1: {s1}"); // compile err; "value borrowed here after move"
}