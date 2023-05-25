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