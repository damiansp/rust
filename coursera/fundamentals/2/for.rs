fn main() {
    for i in 1..10 {
        println!("i: {i}");
    }
    for i in (1..=5).rev() {
        println!("i: {i}");
    }
    let ns = vec![1, 2, 3, 4, 5];
    for n in ns {
        println!("n: {n}");
    }
}