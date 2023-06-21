fn main() {
    let p1 = Point(3, 4);
    let p2 = Point(10, 20);
    let p3 = add(&p1, &p2);  // ownership of p1, p2 still belongs to main (add "borrows" them)
    println!("{p1:?} + {p2:?} = {p3:?}");
}


#[derive(Debug)]
struct Point(i32, i32);


fn add(p1: &Point, p2: &Point) -> Point {
    Point(p1.0 + p2.0, p1.1 + p2.1)
}