fn main() {
    let x: i32 = 5;
    assert_eq!(x, 5);

    let mut y = 1;
    y += 2;
    assert_eq!(y, 3);

    let z: i32 = 10;
    {
        let a: i32 = 5;
        println!("z = {z}; a = {a}");
    }
    println!("z = {z} (a out of scope)");

    define_x();
    println!("Success");
}


fn define_x() {
    let x: &str = "Hello";
    println!("{}, World!", x);
}