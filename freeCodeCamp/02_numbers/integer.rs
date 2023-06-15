fn main() {
    let x: i32 = 5;
    let mut y = 5;
    y = x;
    let z = 10; // i32 by default

    let v: u16 = 38_u8 as u16;

    let a = 5;
    assert_eq!("i32".to_string(), type_of(&a));

    assert_eq!(i8::MAX, 127);
    assert_eq!(u8::MAX, 255);

    let b1 = 251_u16 + 8;
    let b2 = i16::checked_add(251, 8).unwrap();
    println!("{b1}, {b2}");
    println!("Success");
}


fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}