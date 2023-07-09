const THREE_HRS_IN_SECS: u32 = 3 * 60 * 60;


fn main() {
    let mut x = 5;
    println!("x = {x}");
    let x = x + 1;  // no longer mut
    println!("x = {x}");
    {
        let x = 2 * x;
        println!("x (inner) = {x}");
    }
    println!("x = {x}");

    let x = 2.; // f64
    let y: f32 = 3.;

    let sum = 5 + 10;
    let diff = 95.5 - 4.3;
    let prod = 4 * 30;
    let quot = 56.7 / 32.2;
    let trunc = -5 / -3; // -1
    let rem = 43 % 5;
    
    let t = true;
    let f: bool = false;

    let c = 'z';
    let z: char = 'Z';

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (a, b, c) = tup;
    println!("b: {b}");
    let five_o = tup.0;
    println!("five-o: {five_o}");
}
