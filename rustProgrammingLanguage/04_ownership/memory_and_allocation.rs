fn main() {
    let x = 5;
    let y = x;
    let s1 = String::from("Hello");
    let s2 = s1;
    println!("{s1}, world!"); // err: s1 moved to s2
}