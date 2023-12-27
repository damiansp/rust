use std::fmt::Display;


struct Pair<T> {
    x: T,
    y: T
}


impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {x, y}
    }
}


impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x > self.y {
            println!("Largest member is x: {}", self.x);
        } else {
            println!("Largest member is y: {}", self.y);
        }
    }
}