fn main() {
    {
        let p = Point(3, 4);
        println!("x: {}", p.0)
    }
    println!("y: {}", p.1); // err: p no longer in scope
}


struct Point(i32, i32);