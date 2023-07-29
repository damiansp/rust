fn main() {
    //let ref_to_nothing = dangle();    // nope
    let ref_to_something = no_dangle(); // ok
    println!("{ref_to_something}");
}

/*
fn dangle() -> &String {
    let s = String::from("hello");
    &s;
} // s out of scope here
*/


fn no_dangle() -> String {
    //let s = String::from("hello");
    //s
    String::from("hello")
}