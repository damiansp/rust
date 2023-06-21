fn main() {
    let p1 = Point(3, 4);
    let p2 = Point(10, 20);
    let p3 = add(&p1, &p2);  // ownership of p1, p2 still belongs to main (add "borrows" them)
    println!("{p1:?} + {p2:?} = {p3:?}");

    // can have any number of refs, &T or exactly one mutable ref &mut T
    let mut a: i32 = 10;
    let b: &i32 = &a;
    {
        let c: &mut i32 = &mut a; // will not compile
        *c = 20;
    }
}


#[derive(Debug)]
struct Point(i32, i32);


fn add(p1: &Point, p2: &Point) -> Point {
    Point(p1.0 + p2.0, p1.1 + p2.1)
}