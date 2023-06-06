fn main() {
    let a = 10;
    println!("before: {a}");
    {
        let a = "hello";
        println!("inner scope: {a}");
        let a = true;
        println!("shadowed in inner scoper: {a}");
    }
    println!("after: {a}");

    let b = &a;
    let a = a + 1;
    println!("b: {b}, a: {a}");
}