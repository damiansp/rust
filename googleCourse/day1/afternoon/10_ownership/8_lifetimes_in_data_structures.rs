fn main() {
    let text = String::from("The quick brown fox jumps over the lazy dog.");
    let fox = Highlight(&text[4..19]);
    let dog = Highlight(&text[35..43]);
    // erase(text);
    println!("{fox:?}");
    println!("{dog:?}");
}


// if a data type stores borrowed data, it mus be lifetime-annotated
#[derive(Debug)]
struct Highlight<'doc>(&'doc str);


fn erase(text: String) { println!("Deleting {text}..."); }

