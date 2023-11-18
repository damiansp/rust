fn main() {
    let is_even = |x| x % 2 == 0;
    //let is_even = |x: u64| -> bool x % 2 == 0; // err
    let is_even = |x: u64| -> bool { x % 2 == 0 };  // ok
    assert_eq!(is_even(14), true);
}