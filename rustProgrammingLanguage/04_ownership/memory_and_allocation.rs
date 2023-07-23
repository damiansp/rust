fn main() {
    let x = 5;
    let y = x;
    println!("x: {x}; y: {y}");  // works for stack-only types

    let s1 = String::from("Hello");
    let s3 = s1.clone();
    let s2 = s1;
    //println!("{s1}, world!"); // err: s1 moved to s2
    println!("{s2}, world!");
    println!("s3: {s3}");

    let s = String::from("hello");
    takes_ownership(s);  // s's val moves into function, no longer valid here

    let z = 5;
    makes_copy(x);  // x moves in to func, but is Copy type (i32) so still ok to use
}


fn takes_ownership(some_string: String) {
    println!("I own you now, {some_string}");
}


fn makes_copy(some_int: i32) {
    println!("Dealing with a copy of {some_int}");
}