fn main() {
    let mut vec = vec![10, 20];
    vec.push(30);
    let midpoint = vec.len() / 2;
    println!("Midpoint is {}", vec[midpoint]);
    for item in &vec {
        println!("item: {item}");
    }
}