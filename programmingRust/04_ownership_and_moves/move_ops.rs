fn main() {
    let mut s = "Govinda".to_string();
    s = "Siddhartha".to_string();  // Govinda dropped

    let mut r = "Govinda".to_string();
    let t = r;
    r = "Siddhartha".to_string(); 
    println!("t {t}");  // Govinda
}