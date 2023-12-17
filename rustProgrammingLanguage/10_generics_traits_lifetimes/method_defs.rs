fn main() {
    let p = Point{x: 4, y: 5};
    println!("p.x = {}", p.x());
    let p2 = Point{x: "Broadway", y: "Main"};
    let p3 = p1.mixup(p2)
    println!("p3: x = {}, y = {}", p3.x, p3.y);
}


struct Point<X1, Y1> {
    x: X1,
    y: Y1
}


impl<X1, Y1> Point<X1, Y1> {
    fn x(&self) -> &X1 {
        &self.x
    }

    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y1> {
        Point {x: self.x, y: other.y}
    }
}