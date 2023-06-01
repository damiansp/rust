// Basic Types
let a: i8 = 1;  // i16, i32, i64, i128, isize
let b: u8 = 2;  // u16, u32, u64, u128, usize
let f: f32 = 3.0;  // f64
let c: char = 'a';  // any unicode character
let t: bool = true;  // false
let s: &str = "Hello, world!";

// Compound Types
let mut a: [i32; 10] = [42; 10];
let t: (i32, bool) = (17, true);

a[5] = 0;
println!("a: {:?}", a);
println!("t: {:?}", t);
println!("t.0: {}", t.0);


// References
let mut x: i32 = 10;
let ref_x: &mut i32 = &mut x;  // ref to a mutable value
*ref_x += 1;
println!("x: {x}"); // x: 11
let mut ref_var = &a;  // mutable ref (can be reassigned)


// Slices
let arr: [i32; 6] = [1, 2, 3, 4, 5, 6];
println!("arr: {:?}", arr);

let s: &[i32] = &arr[2..4];
println!("s: {:?}", s);  // s: [3, 4]


// String and str
let s1: &str = "World!";  // immutable ref to string slice
println!("s1: {s1}");

let mut s2: String = String::from("Hello, ");  // mutable string buffer
println!("s2: {s2}");;

s2.push_str(s1);
println!("s2: {s2}");

let s3: &str = &s2[8..];
println!("s3: {s3}");


// Functions
fn main() {
    print_fizzbuzz_to(20);
    println!("coin toss: {}", pick_one("heads", "tails"));
    println("cash prize: {}", pick_one(100, 1000));
}


fn print_fizzbuzz_to(n: u32) {
    for i in 1..=n {
        println!("{}", fizzbuzz(i));
    }
}


fn fizzbuzz(n: u32) -> String {
    let fizz = if is_divisible(n, 3) { "fizz" } else { "" };
    let buzz = if is_divisible(n, 5) { "buzz" } else { "" };
    if fizz.is_empty() && buzz.is_empty() {
        return format!(("{n}"));
    }
    format!("{fizz}{buzz}")
}


/// Determine whether the first argument is divisible by the second argument.
/// If the second argument is zero, return false.
fn is_divisible(n: u32, divisor: u32) -> bool {
    if divisor == 0 {
        return false;  // edge case; early return
    } 
    n % divisor == 0   // last expression is returned (if no semicolon)
}


// Methods
fn use_rect() {
    let mut rect = Rectangle {width: 10, height: 5};
    println!("Area: {}", rect.area());
    rect.incr_width(5);
    println!("New Area: {}", rect.area());
}


// Properties
struct Rectangle {
    width: u32,
    height: u32
}


// Methods
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn incr_width(&mut self, delta: u32) {
        self.width += delta;
    }
}


// Function overloading not supported, but generics exist
fn pick_one<T>(a: T, b: T) -> T {
    if std::process::id() % 2 == 0 { a } else { b }
}