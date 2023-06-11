fn main() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12)
    }
    assert_eq!(x, 5);
    let x = 42;
    println!("{x}");

    let mut x: i32 = 1;
    x = 7;
    // Shadowing and re-binding
    let mut x = x;
    x += 3;
    let y = 4;
    // Shadowing
    let y = "I can also be bound to text!";
    println!("Success!")
}