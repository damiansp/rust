fn main() {
    let x = 42;
    let y = x;
    println!("x = {x}; y = {y}");  // ok b/c primitive types are copied by default

    let p1 = Point(3, 4);
    let p2 = p1;
    println!("p1: {p1:?}");
    println!("p2: {p2:?}");
}


#[derive(Copy, Clone, Debug)]  // give copy semantics as default
struct Point(i32, i32);