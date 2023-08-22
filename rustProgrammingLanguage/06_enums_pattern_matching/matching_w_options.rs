fn main() {
    let five = Some(5);
    let six = increment(five);
    let none = increment(None);
}


fn increment(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}