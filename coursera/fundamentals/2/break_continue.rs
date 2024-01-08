fn main() {
    for i in 1..=100 {
        if i % 3 != 0 { continue; }
        println!("i: {i}");
        if i == 48 { break; }
    }
}