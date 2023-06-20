use std::ops::{Range, RangInclusive}

fn main() {
    let mut sum = 0;
    for i in -3 .. 2 {
        sum += i;
    }
    assert!(sum == -5);
    for c in 'a' ..= 'z' {
        println!("{}", c as u8);
    }

    assert_eq!((1..5), Range{start: 1, end: 5});
    assert_eq!((1..=5), RangInclusive::new(1, 5))

    println!("Success")
}