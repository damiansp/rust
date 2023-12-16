fn main() {
    let integer = Point{x: 5, y:10};
    let float = Point{x: 3.0, y: 4.0};
    let both = Point{x: 1, y: 2.3};
}


struct Point<T, U> {
    x: T,
    y: U
}