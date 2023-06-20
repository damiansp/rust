fn main() {
    assert!(1u32 + 2 == 3);
    assert!(1i32 - 2 == -1);
    assert!(1i8 - 2 == -1i8);
    assert!(3 * 50 == 150);  // i32
    assert!(9.6 as f32 / 3.2 as f32 == 3.0 as f32);
    assert!(24 % 5 == 4);
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);
}