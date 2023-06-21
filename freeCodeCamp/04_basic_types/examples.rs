fn main() {
    let x: u32 = 5u32;
    let y = {
        let xsq = x * x;
        let xcub = xsq * x;
        x + xsq + xcub};
    let z = { 2 * x; };  // ()
    println!("x: {x:?}, y: {y:?}, z: {z:?}");
}