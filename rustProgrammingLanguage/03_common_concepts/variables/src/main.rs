const THREE_HRS_IN_SECS: u32 = 3 * 60 * 60;


fn main() {
    let mut x = 5;
    println!("x = {x}");
    x += 1;
    println!("x = {x}");
    {
        let x = 2 * x;
        println!("x (inner) = {x}");
    }
    println!("x = {x}");
}
