fn main() {
    let name = "Hello";
    match name {
        "Good bye" => println!("See ya later");
        "Hello" => println!("Hi there");
        _ => println("Baking powder?");
    }
}