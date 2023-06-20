fn main() {
    let name = String::from("Alice");
    say_hello(name);
    //println!(name);  // error: main no longer owns <name>

    let n2 = String::from("Bob");
    say_hi(&n2);
    println!("{n2}");  // ok

    let n3 = String::from("Charlotte");
    say_hello(n3.clone());
    println!("{n3}");  // ok
}


fn say_hello(name: String) {
    println!("Hello, {name}");
}


fn say_hi(name: &String) {
    println!("Hi, {name}");
}