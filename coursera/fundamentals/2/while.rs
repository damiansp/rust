use std::io;


fn main() {
    let mut i = 0;
    while i < 5 {
        println!("i: {i}");
        i += 1;
    }
    let mut input = String::new();
    while input.trim() != "stop" {
        input.clear();
        println!("Enter a word ('stop' to exit):");
        io::stdin().read_line(&mut input).expect("Failed to read input");
        println!("You entered: {input}");
    }
    println!("Goodbye!");
}