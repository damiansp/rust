fn main() {
    let integer = Point{x: 5, y:10};
    let float = Point{x: 3.0, y: 4.0};
}


struct Point<T> {
    x: T,
    y: T
}