fn main() {
    let p = Point{x: 4, y: 5};
    println!("p.x = {}", p.x());
}


struct Point<T> {
    x: T,
    y: T
}


impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}